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
  window,
  Date
};

use stdweb::web::event::{
  HashChangeEvent,
  DragStartEvent,
  DragEndEvent,
  DragEvent,
  DragDropEvent,
  DragOverEvent
  // DragEnterEvent
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

  qs(".coord").add_event_listener(|event: DragOverEvent| {
    event.prevent_default();
  });

  qs(".coord").add_event_listener(|event: DragDropEvent| {
    event.prevent_default();
    console!(log, "drop!");
    let del = || {
      let el = qs("[data-dragging='true']");
      el.set_attribute("style", "display: none;").unwrap();
      el.set_attribute("data-dragging", "false").unwrap();
    };

    js! {
      var del = @{del};
      if (window.confirm("U sure?")) {
        del();
        del.drop();
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
      qs(".coord").set_text_content("🗑️");
      qs(".coord").set_attribute("style", "font-size: 60px;").unwrap();
      let cursor = qs(".cursor");
      cursor.class_list().remove("zoom").unwrap();
      let clone = el.clone_node(CloneKind::Deep).unwrap();
      el.set_attribute("data-dragging", "true").unwrap();
      cursor.append_child(&clone);
      console!(log, qs(".coord"));
    }));

    link.add_event_listener(drag_event);

    link.add_event_listener(|event: DragEndEvent| {
      let x = f64::from(event.client_x());
      let y = f64::from(event.client_y());

      qs(".coord").set_text_content(&format!("_x: {}, _y: {}", x, y));
      // el.set_attribute("style", "").unwrap();
      let cursor = qs(".cursor");
      let project = qs(".cursor .project");

      let projects = qs("._projects");
      projects.remove_attribute("style");
      qs(".coord").remove_attribute("style");
      cursor.remove_child(&project).unwrap();
    });
  }

  window().add_event_listener(toggle_project_event);

  stdweb::event_loop();
}
