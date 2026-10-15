mod state;
mod handlers;
mod render;
mod input;
mod decorations;
mod elements;

use std::sync::Arc;
use smithay::{
    backend::{
        input::InputEvent,
        renderer::gles::GlesRenderer,
        winit::{self, WinitEvent},
    },
    reexports::wayland_server::{Display, ListeningSocket},
    wayland::{
        compositor::CompositorState,
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
    },
    input::SeatState,
};
use smithay::reexports::winit::event_loop::pump_events::PumpStatus;

use state::{App, ClientState};
use render::render_frame;
use input::{handle_keyboard_input, handle_pointer_motion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    run_winit()
}

fn run_winit() -> Result<(), Box<dyn std::error::Error>> {
    let mut display: Display<App> = Display::new()?;
    let dh = display.handle();

    let compositor_state = CompositorState::new::<App>(&dh);
    let shm_state = ShmState::new::<App>(&dh, vec![]);
    let mut seat_state = SeatState::new();
    let seat = seat_state.new_wl_seat(&dh, "winit");

    let mut state = App::new(
        compositor_state,
        XdgShellState::new::<App>(&dh),
        shm_state,
        seat_state,
        DataDeviceState::new::<App>(&dh),
        seat,
    );

    let listener = ListeningSocket::bind("wayland-5").unwrap();
    let mut clients = Vec::new();

    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;
    let start_time = std::time::Instant::now();
    let keyboard = state.seat.add_keyboard(Default::default(), 200, 200).unwrap();

    unsafe { std::env::set_var("WAYLAND_DISPLAY", "wayland-5") };
    std::process::Command::new("kitty").spawn().ok();

    loop {
        let status = winit.dispatch_new_events(|event| match event {
            WinitEvent::Resized { .. } => {}
            WinitEvent::Input(event) => match event {
                InputEvent::Keyboard { event } => {
                    handle_keyboard_input(&event, &keyboard, &mut state);
                }
                InputEvent::PointerMotionAbsolute { .. } => {
                    handle_pointer_motion(&mut state);
                }
                _ => {}
            },
            _ => (),
        });

        if matches!(status, PumpStatus::Exit(_)) {
            return Ok(());
        }

        let size = backend.window_size();
        
        // ВАЖНО: Обернули в блок {}, чтобы renderer и framebuffer уничтожились 
        // и освободили мутабельный заимствование backend перед вызовом submit()
        {
            let (renderer, mut framebuffer) = backend.bind().unwrap();
            render_frame(
                renderer, 
                &mut framebuffer, 
                &state.xdg_shell_state, 
                size, 
                &start_time,
                &state.windows,
                &state.window_positions,
            );
        }

        if let Ok(Some(stream)) = listener.accept() {
            tracing::info!("Got a client: {:?}", stream);
            let client = display
                .handle()
                .insert_client(stream, Arc::new(ClientState::default()))
                .unwrap();
            clients.push(client);
        }

        display.dispatch_clients(&mut state)?;
        display.flush_clients()?;
        
        backend.submit(Some(&[smithay::utils::Rectangle::from_size(size)])).unwrap();
    }
}