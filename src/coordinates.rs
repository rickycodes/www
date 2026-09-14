use web_sys::MouseEvent;

use crate::constants::COORDINATES_SELECTOR;
use crate::util::{PointerState, listen, query_selector, window};

pub(crate) struct Coordinates;

impl Coordinates {
    pub(crate) fn new() -> Self {
        let coordinates = query_selector(COORDINATES_SELECTOR);
        let state = PointerState::new();

        listen(window().as_ref(), "mousemove", move |event: MouseEvent| {
            let coordinates = coordinates.clone();
            let state = state.clone();
            state.update(
                f64::from(event.client_x()),
                f64::from(event.client_y()),
                move |x, y| {
                    coordinates.set_text_content(Some(&format!("_x: {}, _y: {}", x, y)));
                },
            );
        });

        Self
    }
}
