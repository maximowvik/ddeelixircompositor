use smithay::{
    backend::renderer::{
        element::{
            surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
            Kind,
        },
        gles::{GlesRenderer, GlesTarget},
        utils::draw_render_elements,
        Color32F, Frame, Renderer,
    },
    utils::{Rectangle, Size, Transform},
    wayland::{
        compositor::{SurfaceAttributes, TraversalAction, with_surface_tree_downward},
        shell::xdg::XdgShellState,
    },
};
use std::time::Instant;
use wayland_server::protocol::wl_surface;

pub fn render_frame(
    renderer: &mut GlesRenderer,
    target: &mut GlesTarget,
    xdg_shell_state: &XdgShellState,
    size: Size<i32, smithay::utils::Physical>,
    start_time: &Instant,
) {
    let damage = Rectangle::from_size(size);
    
    let elements: Vec<WaylandSurfaceRenderElement<GlesRenderer>> = xdg_shell_state
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
        })
        .collect();

    if let Ok(mut frame) = renderer.render(target, size, Transform::Flipped180) {
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