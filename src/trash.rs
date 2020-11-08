use stdweb::web::event::{
    DragDropEvent, DragEndEvent, DragEnterEvent, DragEvent, DragLeaveEvent, DragOverEvent,
    DragStartEvent,
};

use stdweb::traits::*;
use stdweb::web::{document, CloneKind, confirm};
use util::{get_range, nl, qs};

use stdweb::unstable::TryInto;
use stdweb::web::{HtmlElement, Node};

fn remove_drag_enter() {
    let coord = qs(".coord");
    coord.class_list().remove("dragenter").unwrap();
    coord.class_list().remove("trash").unwrap();
}

fn reset() {
    remove_drag_enter();

    let drag = document().query_selector(".drag").unwrap();
    if drag.is_some() {
        js!( document.querySelector(".drag").classList.remove("drag"); )
            .try_into()
            .unwrap()
    }
}

fn del() {
    let drag = document().query_selector(".drag").unwrap();
    if drag.is_some() {
        drag.unwrap()
            .set_attribute("style", "display: none;")
            .unwrap()
    }
    reset()
}

pub struct Trash();

impl Trash {
    pub fn new() -> Trash {
        let coord = qs(".coord");
        let cries: [&'static str; 5] = [
            "U sure?",
            "Really?",
            "Y u gotta be this way?",
            "Come on?",
            "Please no",
        ];

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

        coord.add_event_listener(enclose!((cries) move |event: DragDropEvent| {
            event.prevent_default();
            let index = get_range(0 as f64, cries.len() as f64) as usize;
            let okay = confirm(&cries[index].to_string());
            if okay {
                del()
            } else {
                reset()
            }
        }));

        fn bind_link(link: Node) {
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
                js! {
                  /* TODO: fix this hack! */
                  var reset = @{reset};
                  window.setTimeout(reset, 100);
                }
                remove_drag_enter();
                let cursor = qs(".cursor");
                let project = qs(".cursor .project");
                cursor.remove_child(&project).unwrap();
            });
        }

        for link in nl(".project.link, .cv.link") {
            bind_link(link)
        }

        Trash()
    }
}
