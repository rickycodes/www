use stdweb::traits::*;
use stdweb::web::window;
use crate::util::query_selector;

use stdweb::web::event::MouseMoveEvent;

use crate::constants::COORDINATES_SELECTOR;

pub(crate) struct Coordinates;

impl Coordinates {
    pub(crate) fn new() -> Self {
        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            query_selector(COORDINATES_SELECTOR).set_text_content(&format!("_x: {}, _y: {}", x, y));
        };

        window().add_event_listener(mouse_move_event);

        Self
    }
}
