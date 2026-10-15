use smithay::backend::renderer::{
    element::{
        solid::SolidColorRenderElement,
        surface::WaylandSurfaceRenderElement,
        render_elements,
    },
    gles::GlesRenderer,
};

render_elements! {
    pub DeElixirRenderElement<=GlesRenderer>;
    Surface=WaylandSurfaceRenderElement<GlesRenderer>,
    Solid=SolidColorRenderElement,
}