use smithay::{
    backend::input::{InputBackend, InputTime, KeyboardKeyEvent},
    input::keyboard::FilterResult,
};

use crate::state::App;

pub fn handle_keyboard_input<B: InputBackend>(
    event: &dyn KeyboardKeyEvent<B>,
    keyboard: &smithay::input::keyboard::KeyboardHandle<App>,
    state: &mut App,
) {
    keyboard.input::<(), _>(
        state,
        event.key_code(),
        event.state(),
        0.into(),
        InputTime::now(),
        |_, _, _| FilterResult::Forward,
    );
}

pub fn handle_pointer_motion(state: &mut App) {
    if let Some(surface) = state.xdg_shell_state.toplevel_surfaces().iter().next().cloned() {
        let surface = surface.wl_surface().clone();
        state.seat.get_keyboard().unwrap().set_focus(state, Some(surface), 0.into());
    }
}