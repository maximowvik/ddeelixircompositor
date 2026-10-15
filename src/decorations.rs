use smithay::{
    backend::renderer::{
        element::{
            solid::SolidColorRenderElement,
            Id, Kind,
        },
        utils::CommitCounter,
    },
    desktop::Window,
    utils::{Physical, Point, Rectangle, Size},
};

const TITLEBAR_HEIGHT: i32 = 32;
const BUTTON_SIZE: i32 = 20;
const BUTTON_GAP: i32 = 8;

pub struct DecorationElements {
    pub titlebar: SolidColorRenderElement,
    pub close_btn: SolidColorRenderElement,
    pub max_btn: SolidColorRenderElement,
    pub min_btn: SolidColorRenderElement,
}

impl DecorationElements {
    pub fn new(
        window: &Window,
        position: Point<i32, Physical>,
        is_active: bool,
    ) -> Self {
        let window_size = window.geometry().size;

        // Заголовок
        let titlebar_color = if is_active {
            [0.2, 0.2, 0.3, 1.0]
        } else {
            [0.3, 0.3, 0.3, 1.0]
        };

        let titlebar_size = Size::from((window_size.w, TITLEBAR_HEIGHT));
        let titlebar = SolidColorRenderElement::new(
            Id::new(),
            Rectangle::new(position, titlebar_size),
            CommitCounter::default(),
            titlebar_color,
            Kind::Unspecified,
        );

        // Кнопки
        let button_y = position.y + 6;
        let window_right = position.x + window_size.w;

        // Закрыть (красная)
        let close_x = window_right - BUTTON_SIZE - 8;
        let close_btn = SolidColorRenderElement::new(
            Id::new(),
            Rectangle::new(
                Point::new(close_x, button_y),
                Size::from((BUTTON_SIZE, BUTTON_SIZE)),
            ),
            CommitCounter::default(),
            [0.9, 0.3, 0.3, 1.0],
            Kind::Unspecified,
        );

        // Развернуть (жёлтая)
        let max_x = close_x - BUTTON_SIZE - BUTTON_GAP;
        let max_btn = SolidColorRenderElement::new(
            Id::new(),
            Rectangle::new(
                Point::new(max_x, button_y),
                Size::from((BUTTON_SIZE, BUTTON_SIZE)),
            ),
            CommitCounter::default(),
            [0.9, 0.8, 0.3, 1.0],
            Kind::Unspecified,
        );

        // Свернуть (зелёная)
        let min_x = max_x - BUTTON_SIZE - BUTTON_GAP;
        let min_btn = SolidColorRenderElement::new(
            Id::new(),
            Rectangle::new(
                Point::new(min_x, button_y),
                Size::from((BUTTON_SIZE, BUTTON_SIZE)),
            ),
            CommitCounter::default(),
            [0.3, 0.8, 0.3, 1.0],
            Kind::Unspecified,
        );

        Self {
            titlebar,
            close_btn,
            max_btn,
            min_btn,
        }
    }
}