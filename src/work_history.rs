use util::get_hash;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
  document,
  Element
};

use stdweb::web::event::{
  ClickEvent
};

fn scroll_into_view(el: Element) {
  js!{ @(no_return)
    @{el}.scrollIntoView();
  }
}

fn click(el: Element) {
  js!{ @(no_return)
    @{el}.click();
  }
}

fn is_checked(el: Element) -> bool {
  js!( return @{el}.checked; ).try_into().unwrap()
}

pub fn scroll_to() {
  // scroll to #work-history and expand
  let hash = get_hash();
  if hash == "work-history" {
    let input_el = document().query_selector("#cv-toggle").unwrap();
    let scroll_to_el = document().query_selector(&format!("#{}", hash)).unwrap();
    if input_el.is_some() && scroll_to_el.is_some() {
      scroll_into_view(scroll_to_el.unwrap());
      click(input_el.unwrap())
    }
  }
}

pub fn bind_work_toggle() {
  let input_el = document().query_selector("#cv-toggle").unwrap().unwrap();
  input_el.add_event_listener(enclose!( (input_el) move |_: ClickEvent| {
    if is_checked(input_el.clone()) {
      let scroll_to_el = document().query_selector("#work-history").unwrap().unwrap();
      scroll_into_view(scroll_to_el);
    }
  }));
}
