#[macro_use]
extern crate stdweb;

#[macro_use]
mod enclose;

mod slideshows;
mod links;
mod util;
mod toggle_project;
mod canvas;

use toggle_project::toggle;
use util::{qs, get_hash};
use stdweb::traits::*;
use stdweb::web::{
  document,
  window,
  Date
};

use stdweb::web::event::HashChangeEvent;

fn main() {
  stdweb::initialize();
  slideshows::initialize();
  links::initialize();
  canvas::initialize();

  let mut scrolls = Vec::new();

  qs(".date").set_text_content(&Date::new().get_full_year().to_string());

  toggle(&mut scrolls);

  // scroll to #work-history and expand
  let hash = get_hash();
  if hash == "work-history" {
    let input_el = document().query_selector("#cv-toggle").unwrap();
    let scroll_to_el = document().query_selector(&format!("#{}", hash)).unwrap();
    if input_el.is_some() && scroll_to_el.is_some() {
      js!{ @(no_return)
        @{input_el}.click();
        @{scroll_to_el}.scrollIntoView();
      }
    }
  }

  let toggle_project_event = move |_event: HashChangeEvent| {
    toggle(&mut scrolls)
  };

  window().add_event_listener(toggle_project_event);

  stdweb::event_loop();
}
