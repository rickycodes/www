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
use util::qs;
use stdweb::traits::*;
use stdweb::web::{
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

  let toggle_project_event = move |_event: HashChangeEvent| {
    toggle(&mut scrolls)
  };

  window().add_event_listener(toggle_project_event);

  stdweb::event_loop();
}
