#[macro_use]
extern crate stdweb;

#[macro_use]
mod enclose;

mod slideshows;
mod links;
mod util;
mod toggle_project;
mod canvas;
mod work_history;

use work_history::{
  scroll_to,
  bind_work_toggle
};
use toggle_project::toggle;
use util::{
  qs,
  nl
};
use stdweb::traits::*;
use stdweb::web::{
  CloneKind,
  document,
  window,
  Date
};

use stdweb::web::event::{
  HashChangeEvent,
  DragStartEvent,
  DragEndEvent,
  DragEvent,
  DragDropEvent,
  DragOverEvent,
  DragEnterEvent,
  DragLeaveEvent
};

use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;

fn main() {
  stdweb::initialize();
  slideshows::initialize();
  links::initialize();
  canvas::initialize();

  let mut scrolls = Vec::new();

  qs(".date").set_text_content(&Date::new().get_full_year().to_string());

  toggle(&mut scrolls);

  scroll_to();
  bind_work_toggle();

  let toggle_project_event = move |_event: HashChangeEvent| {
    toggle(&mut scrolls)
  };

  let coord = qs(".coord");
  coord.add_event_listener(|event: DragOverEvent| {
    event.prevent_default();
  });

  coord.add_event_listener(|event: DragEnterEvent| {
    event.prevent_default();
    let coord = qs(".coord");
    coord.class_list().add("dragenter").unwrap();
  });

  coord.add_event_listener(|event: DragLeaveEvent| {
    event.prevent_default();
    let coord = qs(".coord");
    coord.class_list().remove("dragenter").unwrap();
  });

  pub fn reset() -> () {
    let coord = qs(".coord");
    coord.class_list().remove("dragenter").unwrap();
    coord.class_list().remove("trash").unwrap();

    let drag = document().query_selector(".drag").unwrap();
    if drag.is_some() {
      js!( document.querySelector(".drag").classList.remove("drag"); ).try_into().unwrap()
    }
  };

  pub fn del() -> () {
    let drag = qs(".drag");
    drag.set_attribute("style", "display: none;").unwrap();
    reset();
  };

  coord.add_event_listener(|event: DragDropEvent| {
    event.prevent_default();
    console!(log, "drop!");

    js! {
      var del = @{del};
      var reset = @{reset};
      if (window.confirm("U sure?")) {
        del();
        del.drop()
      } else {
        reset();
        reset.drop()
      }
    }
  });

  for link in nl(".project.link, .cv.link") {
    let el: HtmlElement = link.clone().try_into().unwrap();
    let drag_event = |event: DragEvent| {
      let cursor = qs(".cursor");
      let x = f64::from(event.client_x());
      let y = f64::from(event.client_y());
      cursor.set_attribute("style", &format!(
        "transform: translate3d({}px,{}px,0);", x, y
      )).unwrap();
    };

    link.add_event_listener(enclose!( (el) move |_event: DragStartEvent| {
      let coord = qs(".coord");
      coord.class_list().add("trash").unwrap();
      let cursor = qs(".cursor");
      cursor.class_list().remove("zoom").unwrap();
      let clone = el.clone_node(CloneKind::Deep).unwrap();
      el.class_list().add("drag").unwrap();
      cursor.append_child(&clone);
      console!(log, coord);
    }));

    link.add_event_listener(drag_event);

    link.add_event_listener(|_event: DragEndEvent| {
      reset();
      let cursor = qs(".cursor");
      let project = qs(".cursor .project");

      let projects = qs("._projects");
      projects.remove_attribute("style");
      cursor.remove_child(&project).unwrap();
    });
  }

  window().add_event_listener(toggle_project_event);

  stdweb::event_loop();
}
