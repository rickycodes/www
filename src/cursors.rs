use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::window;
use stdweb::web::HtmlElement;
use util::{nl, qs};

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
        let cursor = qs(".cursor");
        let close = qs(".close div");

        cursor_hover_events!(close, &cursor, "close");

        for gh in nl(".github.link") {
            cursor_hover_events!(gh, &cursor, "gh");
        }

        for tw in nl(".twitter.link") {
            cursor_hover_events!(tw, &cursor, "tw");
        }

        for project in nl("._projects .project") {
            cursor_hover_events!(project, &cursor, "zoom");
        }

        for prev in nl(".slideshow .prev") {
            cursor_hover_events!(prev, &cursor, "prev");
        }

        for next in nl(".slideshow .next") {
            cursor_hover_events!(next, &cursor, "next");
        }

        for link in nl(".content a:not(.project)") {
            let el: HtmlElement = link.try_into().unwrap();
            el.set_attribute("target", "_blank").unwrap();
            el.set_attribute("rel", "noopener").unwrap();
        }

        let mouse_move_event = move |event: MouseMoveEvent| {
            let x = f64::from(event.client_x());
            let y = f64::from(event.client_y());

            cursor!(qs(".x"), &format!("{}px", x), 0);
            cursor!(qs(".y"), 0, &format!("{}px", y));
            cursor!(
                qs(CURSOR_SELECTOR),
                &format!("{}px", x),
                &format!("{}px", y)
            );
        };

        window().add_event_listener(mouse_move_event);

        Cursors()
    }
}
