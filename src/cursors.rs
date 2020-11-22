use stdweb::traits::*;
use stdweb::web::{window};
use util::{qs};

use stdweb::web::event::{MouseMoveEvent};

use crate::constants::{
    CURSOR_SELECTOR
};
 
macro_rules! cursor {
    ($el:expr, $x:expr, $y:expr) => {
        $el.set_attribute(
            "style",
            &format!("transform: translate3d({},{},0);", $x, $y),
        )
        .unwrap();
    };
}

pub struct Cursors();

impl Cursors {
    pub fn new() -> Cursors {
        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            cursor!(qs(".x"), &format!("{}px", x), 0);
            cursor!(qs(".y"), 0, &format!("{}px", y));
            cursor!(qs(CURSOR_SELECTOR), &format!("{}px", x), &format!("{}px", y));
        };

        window().add_event_listener(mouse_move_event);

        Cursors()
    }
}
