use smithay::{
    backend::input::{ButtonState, MotionEvent, PointerTarget as SmithayPointerTarget},
    desktop::Window,
    input::pointer::{Focus, MotionFocus, PointerHandle, PointerTarget},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point, Serial},
};
use std::collections::HashMap;

use crate::compositor::DeElixirCompositor;

pub struct DeElixirPointerTarget;

impl PointerTarget<DeElixirCompositor> for DeElixirPointerTarget {
    fn enter(
        &self,
        _seat: &smithay::input::Seat<DeElixirCompositor>,
        _data: &mut DeElixirCompositor,
        _event: &MotionEvent,
    ) {
        // Ничего не делаем при входе
    }

    fn motion(
        &self,
        _seat: &smithay::input::Seat<DeElixirCompositor>,
        data: &mut DeElixirCompositor,
        event: &MotionEvent,
    ) {
        // Находим surface под курсором
        if let Some((surface, window_pos)) = data.surface_under(event.location) {
            // ТРАНСФОРМАЦИЯ КООРДИНАТ: вычитаем позицию окна
            let surface_location = event.location - window_pos.to_f64();
            
            // Отправляем событие в surface
            data.pointer.motion(data, event.time, surface_location);
        }
    }

    fn button(
        &self,
        _seat: &smithay::input::Seat<DeElixirCompositor>,
        data: &mut DeElixirCompositor,
        event: &smithay::backend::input::ButtonEvent,
    ) {
        if let Some((surface, window_pos)) = data.surface_under(event.location) {
            // Проверяем, не попали ли в декорации (заголовок)
            if data.is_in_titlebar(event.location, &surface) {
                // Начинаем перетаскивание окна
                data.start_interactive_move(surface, event);
            } else {
                // Передаём клик в окно
                let surface_location = event.location - window_pos.to_f64();
                data.pointer.button(data, event.time, event.button, event.state);
            }
        }
    }

    fn leave(
        &self,
        _seat: &smithay::input::Seat<DeElixirCompositor>,
        _data: &mut DeElixirCompositor,
        _serial: Serial,
        _time: u32,
    ) {
        // Убираем фокус
    }
}

impl DeElixirCompositor {
    // Находим surface под курсором
    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(WlSurface, Point<i32, Logical>)> {
        // Сначала проверяем layer-shell поверхности (панели)
        for layer in &self.layers {
            let layer_bbox = layer.geometry();
            if layer_bbox.contains(pos.to_i32_round()) {
                let surface = layer.wl_surface();
                let offset = layer_bbox.loc;
                return Some((surface, offset));
            }
        }
        
        // Потом проверяем окна (в обратном порядке — сверху вниз)
        for window in self.windows.iter().rev() {
            let window_pos = self.window_positions
                .get(&window.toplevel().wl_surface())
                .copied()
                .unwrap_or_default();
            
            let window_bbox = Rectangle::from_loc_and_size(
                window_pos,
                window.geometry().size,
            );
            
            if window_bbox.contains(pos.to_i32_round()) {
                let surface = window.toplevel().wl_surface();
                return Some((surface, window_pos));
            }
        }
        
        None
    }
    
    // Проверяем, попали ли в заголовок окна
    pub fn is_in_titlebar(
        &self,
        pos: Point<f64, Logical>,
        surface: &WlSurface,
    ) -> bool {
        if let Some(window_pos) = self.window_positions.get(surface) {
            let titlebar_height = 32;
            pos.y < (window_pos.y + titlebar_height) as f64
        } else {
            false
        }
    }
    
    // Начинаем перетаскивание окна
    pub fn start_interactive_move(
        &mut self,
        surface: WlSurface,
        event: &smithay::backend::input::ButtonEvent,
    ) {
        // Сохраняем начальную позицию
        let start_pos = self.window_positions.get(&surface).copied().unwrap_or_default();
        let start_cursor = event.location;
        
        // В реальном композиторе здесь нужно начать grab
        // и обновлять позицию окна в каждом motion событии
        log::info!("Start moving window at {:?}", start_pos);
    }
}