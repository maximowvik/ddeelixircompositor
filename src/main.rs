mod compositor;
mod input;
mod render;
mod wayland;
mod panels;

use compositor::DeElixirCompositor;

fn main() {
    env_logger::init();
    
    let mut compositor = DeElixirCompositor::new();
    compositor.run();
}