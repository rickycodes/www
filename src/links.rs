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
  for link in nl(".content a:not(.project)") {
    let el: HtmlElement = link.clone().try_into().unwrap();
    el.set_attribute("target", "_blank").unwrap();
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
