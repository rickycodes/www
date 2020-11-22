use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::window;
use stdweb::web::HtmlElement;
use util::{node_list, query_selector};

use stdweb::web::event::{MouseMoveEvent, MouseOutEvent, MouseOverEvent};

use crate::constants::CURSOR_SELECTOR;

macro_rules! cursor_hover_events {
    ($el:expr, $cursor:expr, $class:expr) => {
      let c = $cursor;
      $el.add_event_listener(enclose!( (c) move |_event: MouseOverEvent| {
        c.class_list().add( $class ).unwrap();
      }));

      $el.add_event_listener(enclose!( (c) move |_event: MouseOutEvent| {
        c.class_list().remove( $class ).unwrap();
      }));
    }
}

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
        let cursor = query_selector(".cursor");
        let close = query_selector(".close div");

        cursor_hover_events!(close, &cursor, "close");

        for gh in node_list(".github.link") {
            cursor_hover_events!(gh, &cursor, "gh");
        }

        for tw in node_list(".twitter.link") {
            cursor_hover_events!(tw, &cursor, "tw");
        }

        for project in node_list("._projects .project") {
            cursor_hover_events!(project, &cursor, "zoom");
        }

        for prev in node_list(".slideshow .prev") {
            cursor_hover_events!(prev, &cursor, "prev");
        }

        for next in node_list(".slideshow .next") {
            cursor_hover_events!(next, &cursor, "next");
        }

        for link in node_list(".content a:not(.project)") {
            let el: HtmlElement = link.try_into().unwrap();
            el.set_attribute("target", "_blank").unwrap();
            el.set_attribute("rel", "noopener").unwrap();
        }

        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            cursor!(query_selector(".x"), &format!("{}px", x), 0);
            cursor!(query_selector(".y"), 0, &format!("{}px", y));
            cursor!(
                query_selector(CURSOR_SELECTOR),
                &format!("{}px", x),
                &format!("{}px", y)
            );
        };

        window().add_event_listener(mouse_move_event);

        Cursors()
    }
}
