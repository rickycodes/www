use stdweb::web::event::{
    DragDropEvent, DragEndEvent, DragEnterEvent, DragEvent, DragLeaveEvent, DragOverEvent,
    DragStartEvent,
};

use stdweb::traits::*;
use stdweb::web::{document, CloneKind};
use util::{nl, qs};

use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;

fn reset() -> () {
    let coord = qs(".coord");
    coord.class_list().remove("dragenter").unwrap();
    coord.class_list().remove("trash").unwrap();
    let drag = document().query_selector(".drag").unwrap();
    if drag.is_some() {
        js!( document.querySelector(".drag").classList.remove("drag"); )
            .try_into()
            .unwrap()
    }
}

fn del() -> () {
    let drag = qs(".drag");
    drag.set_attribute("style", "display: none;").unwrap();
    reset();
}

pub struct Trash();

impl Trash {
    pub fn new() -> Trash {
        let coord = qs(".coord");

        /* drag and drop stuffs */
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

        coord.add_event_listener(|event: DragDropEvent| {
            event.prevent_default();

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
                cursor
                    .set_attribute(
                        "style",
                        &format!("transform: translate3d({}px,{}px,0);", x, y),
                    )
                    .unwrap();
            };

            link.add_event_listener(enclose!( (el) move |_event: DragStartEvent| {
              let coord = qs(".coord");
              coord.class_list().add("trash").unwrap();
              let cursor = qs(".cursor");
              cursor.class_list().remove("zoom").unwrap();
              let clone = el.clone_node(CloneKind::Deep).unwrap();
              el.class_list().add("drag").unwrap();
              cursor.append_child(&clone);
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

        Trash()
    }
}
