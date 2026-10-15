use smithay::{
    desktop::Window,
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
    },
    input::{Seat, SeatState},
    utils::Point,
};
use wayland_server::{
    backend::{ClientData, ClientId, DisconnectReason}
};

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {
        tracing::info!("Client initialized");
    }

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        tracing::info!("Client disconnected");
    }
}

pub struct App {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
    pub seat: Seat<Self>,
    pub windows: Vec<Window>,
    pub window_positions: Vec<Point<i32, smithay::utils::Logical>>,
    pub next_window_offset: i32,
}

impl App {
    pub fn new(
        compositor_state: CompositorState,
        xdg_shell_state: XdgShellState,
        shm_state: ShmState,
        seat_state: SeatState<Self>,
        data_device_state: DataDeviceState,
        seat: Seat<Self>,
    ) -> Self {
        Self {
            compositor_state,
            xdg_shell_state,
            shm_state,
            seat_state,
            data_device_state,
            seat,
            windows: Vec::new(),
            window_positions: Vec::new(),
            next_window_offset: 30,
        }
    }
}