use util::{nl,qs};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;

use stdweb::web::event::{
  MouseOverEvent,
  MouseOutEvent
};

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

pub fn initialize() {
  let info = qs(".info");

  let cursor = qs(".cursor");
  let close = qs(".close div");

  cursor_hover_events!(close, &cursor, "close");

  for prev in nl(".slideshow .prev") {
    cursor_hover_events!(prev, &cursor, "prev");
  }

  for next in nl(".slideshow .next") {
    cursor_hover_events!(next, &cursor, "next");
  }

  for link in nl(".content a:not(.project)") {
    let el: HtmlElement = link.clone().try_into().unwrap();
    el.set_attribute("target", "_blank").unwrap();
    el.set_attribute("rel", "noopener").unwrap();
    link.add_event_listener(enclose!( (el, info) move |_event: MouseOverEvent| {
      let title = el.get_attribute("title");
      if title != None {
        let title_str = title.unwrap();
        info.set_text_content(&title_str);
        info.class_list().remove( "hidden" ).unwrap();
      }
    }));

    link.add_event_listener(enclose!( (info) move |_event: MouseOutEvent| {
      info.class_list().add( "hidden" ).unwrap();
    }));
  }
}
