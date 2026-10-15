use smithay::{
    backend::{
        input::InputBackend,
        renderer::{
            element::{
                memory::MemoryRenderBufferRenderElement,
                surface::WaylandSurfaceRenderElement,
                Kind,
            },
            gles::GlesRenderer,
            ImportAll, ImportMem, Renderer,
        },
        winit::{WinitBackend, WinitEvent, WinitInput},
    },
    delegate_compositor, delegate_data_device, delegate_keyboard_shortcuts_inhibit,
    delegate_output, delegate_pointer_constraints, delegate_pointer_grabbing,
    delegate_primary_selection, delegate_relative_pointer, delegate_seat,
    delegate_shm, delegate_viewporter, delegate_wl_shell, delegate_xdg_shell,
    desktop::{
        space::SpaceState, utils::SurfaceTree, LayerSurface, Window,
    },
    input::{
        keyboard::{KeyboardHandle, XkbConfig},
        pointer::{PointerHandle, PointerTarget},
    },
    output::Output,
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            EventLoop, LoopSignal,
        },
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            Display, DisplayHandle,
        },
    },
    utils::{Clock, Logical, Point, Rectangle, Scale, Transform},
    wayland::{
        compositor::{self, CompositorClientState, CompositorState},
        data_device::DataDeviceState,
        keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState,
        output::OutputManagerState,
        pointer_constraints::PointerConstraintsState,
        primary_selection::PrimarySelectionState,
        relative_pointer::RelativePointerManagerState,
        seat::SeatState,
        shell::{
            wlr_layer::{Layer, WlrLayerShellState},
            xdg::XdgShellState,
        },
        shm::ShmState,
        viewporter::ViewporterState,
    },
};
use std::{collections::HashMap, time::Duration};
use wayland_server::protocol::wl_surface::WlSurface;

use crate::input::pointer::DeElixirPointerTarget;
use crate::panels::{TopBar, Dock};
use crate::render::decorations::DecorationRenderer;

pub struct DeElixirCompositor {
    // Wayland display
    pub display: Display<Self>,
    pub display_handle: DisplayHandle,
    
    // Backend
    pub backend: WinitBackend,
    pub renderer: GlesRenderer,
    
    // State management
    pub space: SpaceState,
    pub windows: Vec<Window>,
    pub window_positions: HashMap<WlSurface, Point<i32, Logical>>,
    
    // Input
    pub keyboard: KeyboardHandle<Self>,
    pub pointer: PointerHandle<Self>,
    pub seat_state: SeatState<Self>,
    
    // Wayland protocols
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState,
    pub output_manager_state: OutputManagerState,
    pub pointer_constraints_state: PointerConstraintsState,
    pub primary_selection_state: PrimarySelectionState,
    pub relative_pointer_state: RelativePointerManagerState,
    pub shm_state: ShmState,
    pub viewporter_state: ViewporterState,
    pub xdg_shell_state: XdgShellState,
    pub layer_shell_state: WlrLayerShellState,
    
    // Layers (панели)
    pub layers: Vec<LayerSurface>,
    
    // Panels
    pub top_bar: Option<TopBar>,
    pub dock: Option<Dock>,
    
    // Output
    pub output: Output,
    
    // Loop signal
    pub loop_signal: LoopSignal,
}

impl DeElixirCompositor {
    pub fn new() -> Self {
        // Создаём Wayland display
        let mut display = Display::<Self>::new().unwrap();
        let display_handle = display.handle();
        
        // Создаём Winit backend (для тестирования в окне)
        let (mut backend, mut winit) = WinitBackend::new().unwrap();
        
        // Создаём рендерер
        let renderer = unsafe { GlesRenderer::new(backend.egl_context().clone()) }.unwrap();
        
        // Создаём output
        let output = Output::new(
            "eDP-1".to_string(),
            PhysicalProperties {
                size: (1920, 1080).into(),
                subpixel: Subpixel::Unknown,
                make: "Generic".into(),
                model: "Monitor".into(),
            },
        );
        output.change_current_state(
            Some(Mode {
                size: (1920, 1080).into(),
                refresh: 60000,
            }),
            None,
            None,
            None,
        );
        output.set_preferred(Mode {
            size: (1920, 1080).into(),
            refresh: 60000,
        });
        
        // Создаём event loop
        let mut event_loop = EventLoop::<Self>::try_new().unwrap();
        let loop_signal = event_loop.get_signal();
        
        // Инициализируем Wayland протоколы
        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let data_device_state = DataDeviceState::new::<Self>(&display_handle);
        let keyboard_shortcuts_inhibit_state = KeyboardShortcutsInhibitState::new::<Self>(&display_handle);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&display_handle);
        let pointer_constraints_state = PointerConstraintsState::new::<Self>(&display_handle);
        let primary_selection_state = PrimarySelectionState::new::<Self>(&display_handle);
        let relative_pointer_state = RelativePointerManagerState::new::<Self>(&display_handle);
        let shm_state = ShmState::new::<Self>(&display_handle, vec![]);
        let viewporter_state = ViewporterState::new::<Self>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let layer_shell_state = WlrLayerShellState::new::<Self>(&display_handle);
        
        // Создаём seat (устройства ввода)
        let mut seat_state = SeatState::new();
        let mut seat = seat_state.new_wl_seat(&display_handle, "seat0");
        
        let keyboard = seat.add_keyboard(XkbConfig::default(), 200, 25).unwrap();
        let pointer = seat.add_pointer();
        
        // Создаём композитор
        let mut compositor = Self {
            display,
            display_handle,
            backend,
            renderer,
            space: SpaceState::new(),
            windows: Vec::new(),
            window_positions: HashMap::new(),
            keyboard,
            pointer,
            seat_state,
            compositor_state,
            data_device_state,
            keyboard_shortcuts_inhibit_state,
            output_manager_state,
            pointer_constraints_state,
            primary_selection_state,
            relative_pointer_state,
            shm_state,
            viewporter_state,
            xdg_shell_state,
            layer_shell_state,
            layers: Vec::new(),
            top_bar: None,
            dock: None,
            output,
            loop_signal,
        };
        
        // Инициализируем панели
        compositor.top_bar = Some(TopBar::new());
        compositor.dock = Some(Dock::new());
        
        compositor
    }
    
    pub fn run(&mut self) {
        // Запускаем event loop
        let mut event_loop = EventLoop::<Self>::try_new().unwrap();
        
        // Добавляем таймер для обновления часов
        event_loop.handle().insert_source(
            Timer::from_duration(Duration::from_secs(1)),
            |_, _, compositor| {
                if let Some(ref mut top_bar) = compositor.top_bar {
                    top_bar.update_clock();
                }
                TimeoutAction::ToDuration(Duration::from_secs(1))
            },
        ).unwrap();
        
        // Главный цикл
        event_loop.run(None, self, |event, _, compositor| match event {
            WinitEvent::Resized { size, scale_factor } => {
                // Обработка изменения размера
            }
            WinitEvent::Input(event) => {
                // Обработка ввода
                compositor.process_input(event);
            }
            WinitEvent::Redraw => {
                // Рендеринг
                compositor.render();
            }
            _ => {}
        }).unwrap();
    }
    
    fn process_input(&mut self, event: WinitInput) {
        // Делегируем обработку ввода
        input::handle_event(self, event);
    }
    
    fn render(&mut self) {
        // Рендеринг сцены
        render::render_scene(self);
    }
}

// Делегирование Wayland протоколов
delegate_compositor!(DeElixirCompositor);
delegate_data_device!(DeElixirCompositor);
delegate_keyboard_shortcuts_inhibit!(DeElixirCompositor);
delegate_output!(DeElixirCompositor);
delegate_pointer_constraints!(DeElixirCompositor);
delegate_pointer_grabbing!(DeElixirCompositor);
delegate_primary_selection!(DeElixirCompositor);
delegate_relative_pointer!(DeElixirCompositor);
delegate_seat!(DeElixirCompositor);
delegate_shm!(DeElixirCompositor);
delegate_viewporter!(DeElixirCompositor);
delegate_wl_shell!(DeElixirCompositor);
delegate_xdg_shell!(DeElixirCompositor);

impl ClientData for DeElixirCompositor {
    fn initialized(&self, client_id: ClientId) {}
    fn disconnected(&self, client_id: ClientId, reason: DisconnectReason) {}
}