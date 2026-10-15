use smithay::{
    backend::renderer::{
        element::{
            surface::render_elements_from_surface_tree,
            Kind,
        },
        gles::{GlesRenderer, GlesTarget},
        utils::draw_render_elements,
        Color32F, Frame, Renderer,
    },
    desktop::Window,
    utils::{Logical, Point, Rectangle, Size},
    wayland::{
        compositor::{SurfaceAttributes, TraversalAction, with_surface_tree_downward},
        shell::xdg::XdgShellState,
    },
};
use std::time::Instant;
use wayland_server::protocol::wl_surface;

use crate::decorations::DecorationElements;
use crate::elements::DeElixirRenderElement;

pub fn render_frame(
    renderer: &mut GlesRenderer,
    target: &mut GlesTarget,
    xdg_shell_state: &XdgShellState,
    size: Size<i32, smithay::utils::Physical>,
    start_time: &Instant,
    windows: &[Window],
    window_positions: &[Point<i32, Logical>],
) {
    let damage = Rectangle::from_size(size);

    // Сначала декорации (чтобы они были поверх окон)
    let mut decorations_elements: Vec<DeElixirRenderElement> = Vec::new();
    for (i, window) in windows.iter().enumerate() {
        if let Some(position) = window_positions.get(i) {
            let is_active = i == windows.len() - 1;
            let position_physical = position.to_physical(1);
            let decorations = DecorationElements::new(window, position_physical, is_active);
            decorations_elements.push(decorations.titlebar.into());
            decorations_elements.push(decorations.close_btn.into());
            decorations_elements.push(decorations.max_btn.into());
            decorations_elements.push(decorations.min_btn.into());
        }
    }

    // Собираем элементы окон и добавляем декорации первыми
    let mut elements: Vec<DeElixirRenderElement> = decorations_elements;
    elements.extend(xdg_shell_state
        .toplevel_surfaces()
        .iter()
        .flat_map(|surface| {
            render_elements_from_surface_tree(
                renderer,
                surface.wl_surface(),
                (0, 0),
                1.0,
                1.0,
                Kind::Unspecified,
            )
        }));

    if let Ok(mut frame) = renderer.render(target, size, smithay::utils::Transform::Flipped180) {
        let _ = frame.clear(Color32F::new(0.1, 0.0, 0.0, 1.0), &[damage]);
        let _ = draw_render_elements(&mut frame, 1.0, &elements, &[damage]);
        let _ = frame.finish();
    }

    let elapsed = start_time.elapsed().as_millis() as u32;
    for surface in xdg_shell_state.toplevel_surfaces() {
        send_frames_surface_tree(surface.wl_surface(), elapsed);
    }
}

fn send_frames_surface_tree(surface: &wl_surface::WlSurface, time: u32) {
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |_surf, states, &()| {
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}