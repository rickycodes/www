use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;
use util::{nl, qs};

use stdweb::web::event::{MouseOutEvent, MouseOverEvent};

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

pub struct Links();

pub fn show_info(str: &str, el: HtmlElement, info: HtmlElement) {
  let attr = el.get_attribute(str);
  if attr != None {
    let attr_str = attr.unwrap();
    info.set_text_content(&attr_str);
    info.class_list().remove( "hidden" ).unwrap();
  }
}

pub fn hide_info(info: HtmlElement) {
  info.class_list().add( "hidden" ).unwrap();
}

impl Links {
    pub fn new() -> Links {
        let info = qs(".info");

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

        for link in nl(".content a[title], .content label[name]") {
            let el: HtmlElement = link.clone().try_into().unwrap();
            link.add_event_listener(enclose!( (el, info) move |_event: MouseOverEvent| {
              show_info("name", el.clone(), info.clone());
              show_info("title", el.clone(), info.clone());
            }));

            link.add_event_listener(enclose!( (info) move |_event: MouseOutEvent| {
              hide_info(info.clone());
            }));
        }

        Links()
    }
}
