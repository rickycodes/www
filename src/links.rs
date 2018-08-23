use util::{nl,qs};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;

use stdweb::web::event::{
  MouseOverEvent,
  MouseOutEvent
};

pub fn initialize() {
  let info = qs(".info");

  let cursor = qs(".cursor");
  let close = qs(".close");

  close.add_event_listener(enclose!( (cursor) move |_event: MouseOverEvent| {
    cursor.class_list().add( "close" ).unwrap();
  }));

  close.add_event_listener(enclose!( (cursor) move |_event: MouseOutEvent| {
    cursor.class_list().remove( "close" ).unwrap();
  }));

  for prev in nl(".slideshow .prev") {
    console!(log, &prev);
    prev.add_event_listener(enclose!( (cursor) move |_event: MouseOverEvent| {
      cursor.class_list().add( "prev" ).unwrap();
    }));

    prev.add_event_listener(enclose!( (cursor) move |_event: MouseOutEvent| {
      cursor.class_list().remove( "prev" ).unwrap();
    }));
  }

  for next in nl(".slideshow .next") {
    next.add_event_listener(enclose!( (cursor) move |_event: MouseOverEvent| {
      cursor.class_list().add( "next" ).unwrap();
    }));

    next.add_event_listener(enclose!( (cursor) move |_event: MouseOutEvent| {
      cursor.class_list().remove( "next" ).unwrap();
    }));
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
