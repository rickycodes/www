use stdweb::traits::*;
use stdweb::web::window;
use util::query_selector;

use stdweb::web::event::MouseMoveEvent;

pub struct Coordinates();

impl Coordinates {
    pub fn new() -> Coordinates {
        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            query_selector(".coord > div").set_text_content(&format!("_x: {}, _y: {}", x, y));
        };

        window().add_event_listener(mouse_move_event);

        Coordinates()
    }
}
