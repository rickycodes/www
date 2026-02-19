use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{MouseMoveEvent, MouseOutEvent, MouseOverEvent};
use stdweb::web::window;
use stdweb::web::HtmlElement;
use crate::util::{node_list, query_selector};

use crate::constants::{
    CLOSE, CLOSE_SELECTOR, CURSORS_ATTRIBUTES, CURSOR_SELECTOR, STYLE, X_SELECTOR, Y_SELECTOR, ZERO,
};

struct Cursor;

impl Cursor {
    pub(crate) fn new(el: HtmlElement, cursor: &HtmlElement, classname: &'static str) -> Self {
        el.add_event_listener(
            enclose!( (cursor, classname) move |_event: MouseOverEvent| {
                cursor.class_list().add( classname ).unwrap();
            }),
        );

        el.add_event_listener(enclose!( (cursor, classname) move |_event: MouseOutEvent| {
            cursor.class_list().remove( classname ).unwrap();
        }));

        Cursor
    }
}

fn set_cursor_coordinates(el: &HtmlElement, x: &str, y: &str) {
    el.set_attribute(STYLE, &format!("transform: translate3d({},{},0);", x, y))
        .unwrap()
}

pub(crate) struct Cursors;

impl Cursors {
    pub(crate) fn new() -> Self {
        let cursor_element = query_selector(CURSOR_SELECTOR);
        let close = query_selector(CLOSE_SELECTOR);
        let x_element = query_selector(X_SELECTOR);
        let y_element = query_selector(Y_SELECTOR);
        let cursor_for_move = cursor_element.clone();

        Cursor::new(close, &cursor_element, CLOSE);

        for cursor_attribute in &CURSORS_ATTRIBUTES {
            for cursor in node_list(cursor_attribute.selector) {
                Cursor::new(
                    cursor.try_into().unwrap(),
                    &cursor_element,
                    cursor_attribute.classname,
                );
            }
        }

        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            self::set_cursor_coordinates(&x_element, &format!("{}px", x), ZERO);
            self::set_cursor_coordinates(&y_element, ZERO, &format!("{}px", y));
            self::set_cursor_coordinates(
                &cursor_for_move,
                &format!("{}px", x),
                &format!("{}px", y),
            );
        };

        window().add_event_listener(mouse_move_event);

        Self
    }
}
