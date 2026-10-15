use smithay::{
    backend::renderer::{
        element::{
            memory::MemoryRenderBufferRenderElement,
            Kind,
        },
        gles::GlesRenderer,
        Renderer,
    },
    desktop::Window,
    utils::{Logical, Point, Rectangle, Scale, Transform},
};
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache};
use image::{ImageBuffer, Rgba};

pub struct DecorationRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl DecorationRenderer {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }
    
    // Рендерим заголовок окна
    pub fn render_titlebar(
        &mut self,
        renderer: &mut GlesRenderer,
        window: &Window,
        position: Point<i32, Logical>,
    ) {
        let titlebar_height = 32;
        let window_size = window.geometry().size;
        
        // Создаём буфер для заголовка
        let mut buffer = ImageBuffer::from_pixel(
            window_size.w as u32,
            titlebar_height as u32,
            Rgba([30, 30, 46, 255]), // Цвет фона заголовка
        );
        
        // Рисуем кнопки (×, □, −)
        self.draw_buttons(&mut buffer, window_size.w);
        
        // Рисуем заголовок окна
        self.draw_title(&mut buffer, "Window Title", window_size.w);
        
        // Конвертируем в render element и рисуем
        // (в реальном коде нужно создать MemoryRenderBufferRenderElement)
    }
    
    fn draw_buttons(&self, buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, width: i32) {
        let button_size = 20;
        let button_gap = 8;
        let y_offset = 6;
        
        // Кнопка закрытия (×)
        let close_x = width - button_size - 8;
        self.draw_circle(buffer, close_x, y_offset, button_size, [243, 139, 168, 255]);
        
        // Кнопка развернуть (□)
        let max_x = close_x - button_size - button_gap;
        self.draw_square(buffer, max_x, y_offset, button_size, [249, 226, 175, 255]);
        
        // Кнопка свернуть (−)
        let min_x = max_x - button_size - button_gap;
        self.draw_line(buffer, min_x, y_offset + button_size / 2, button_size, [166, 227, 161, 255]);
    }
    
    fn draw_circle(&self, buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32, size: i32, color: [u8; 4]) {
        let center_x = x + size / 2;
        let center_y = y + size / 2;
        let radius = size / 2;
        
        for dy in 0..size {
            for dx in 0..size {
                let dist = ((dx - radius) * (dx - radius) + (dy - radius) * (dy - radius)) as f32;
                if dist <= (radius * radius) as f32 {
                    let px = x + dx;
                    let py = y + dy;
                    if px >= 0 && px < buffer.width() as i32 && py >= 0 && py < buffer.height() as i32 {
                        buffer.put_pixel(px as u32, py as u32, Rgba(color));
                    }
                }
            }
        }
    }
    
    fn draw_square(&self, buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32, size: i32, color: [u8; 4]) {
        for dy in 0..size {
            for dx in 0..size {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < buffer.width() as i32 && py >= 0 && py < buffer.height() as i32 {
                    buffer.put_pixel(px as u32, py as u32, Rgba(color));
                }
            }
        }
    }
    
    fn draw_line(&self, buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32, width: i32, color: [u8; 4]) {
        let line_height = 3;
        for dy in 0..line_height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < buffer.width() as i32 && py >= 0 && py < buffer.height() as i32 {
                    buffer.put_pixel(px as u32, py as u32, Rgba(color));
                }
            }
        }
    }
    
    fn draw_title(&self, buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, title: &str, width: i32) {
        // Используем cosmic-text для рендеринга текста
        let metrics = Metrics::new(14.0, 18.0);
        let mut text_buffer = Buffer::new(&mut self.font_system, metrics);
        text_buffer.set_size(&mut self.font_system, Some(width as f32 - 100.0), Some(32.0));
        text_buffer.set_text(&mut self.font_system, title, Attrs::new().family(Family::SansSerif), Shaping::Advanced);
        
        // Рендерим текст в буфер
        // (в реальном коде нужно использовать swash_cache для растеризации)
    }
}