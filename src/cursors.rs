use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::window;
use stdweb::web::HtmlElement;
use util::{node_list, query_selector};
use stdweb::web::event::{MouseMoveEvent, MouseOutEvent, MouseOverEvent};

use crate::constants::CURSOR_SELECTOR;

struct Cursor;

struct CursorAttributes {
    selector: &'static str,
    classname: &'static str
}

impl Cursor {
    pub fn new(el: HtmlElement, cursor: &HtmlElement, classname: &'static str) -> Self {
        el.add_event_listener(enclose!( (cursor, classname) move |_event: MouseOverEvent| {
            cursor.class_list().add( classname ).unwrap();
        }));
    
        el.add_event_listener(enclose!( (cursor, classname) move |_event: MouseOutEvent| {
            cursor.class_list().remove( classname ).unwrap();
        }));

        Cursor
    }
}

fn set_cursor_coordinates(el: HtmlElement, x: &str, y: &str) {
    el.set_attribute("style", &format!("transform: translate3d({},{},0);", x, y))
        .unwrap()
}

pub struct Cursors();

impl Cursors {
    pub fn new() -> Cursors {
        let cursor_element = query_selector(".cursor");
        let close = query_selector(".close div");

        Cursor::new(close, &cursor_element, "close");

        let cursors = [
            CursorAttributes { selector: ".github.link", classname: "gh" },
            CursorAttributes { selector: ".twitter.link", classname: "tw" },
            CursorAttributes { selector: "._projects .project", classname: "zoom" },
            CursorAttributes { selector: ".slideshow .prev", classname: "prev" },
            CursorAttributes { selector: ".slideshow .next", classname: "next" },
        ];

        for cursor in &cursors {
            for c in node_list(cursor.selector) {
                Cursor::new(c.try_into().unwrap(), &cursor_element, cursor.classname);
            }
        }

        for link in node_list(".content a:not(.project)") {
            let el: HtmlElement = link.try_into().unwrap();
            el.set_attribute("target", "_blank").unwrap();
            el.set_attribute("rel", "noopener").unwrap();
        }

        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            set_cursor_coordinates(query_selector(".x"), &format!("{}px", x), "0");
            set_cursor_coordinates(query_selector(".y"), "0", &format!("{}px", y));
            set_cursor_coordinates(
                query_selector(CURSOR_SELECTOR),
                &format!("{}px", x),
                &format!("{}px", y)
            );
        };

        window().add_event_listener(mouse_move_event);

        Cursors()
    }
}
