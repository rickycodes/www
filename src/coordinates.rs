use crate::util::{query_selector, PointerState};
use stdweb::traits::*;
use stdweb::web::window;

use stdweb::web::event::MouseMoveEvent;

use crate::constants::COORDINATES_SELECTOR;

pub(crate) struct Coordinates;

impl Coordinates {
    pub(crate) fn new() -> Self {
        let coordinates = query_selector(COORDINATES_SELECTOR);
        let state = PointerState::new();
        let state_for_event = state.clone();

        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());
            let coordinates = coordinates.clone();
            state_for_event.clone().update(x, y, move |x, y| {
                coordinates.set_text_content(&format!("_x: {}, _y: {}", x, y));
            });
        };

        window().add_event_listener(mouse_move_event);

        Self
    }
}
