use stdweb::web::event::{
    DragDropEvent, DragEndEvent, DragEnterEvent, DragEvent, DragLeaveEvent, DragOverEvent,
    DragStartEvent,
};

use stdweb::traits::*;
use stdweb::web::{confirm, document, CloneKind};
use util::{get_range, node_list, query_selector};

use stdweb::unstable::TryInto;
use stdweb::web::{HtmlElement, Node};

use crate::constants::{
    COORDINATE_SELECTOR, CRIES, CURSOR_PROJECT_SELECTOR, CURSOR_SELECTOR, DISPLAY_NONE, DRAG,
    DRAG_ENTER, DRAG_SELECTOR, LINK_SELECTOR, STYLE, TRASH, ZOOM,
};

fn remove_drag_enter() {
    let coord = query_selector(COORDINATE_SELECTOR);
    coord.class_list().remove(DRAG_ENTER).unwrap();
    coord.class_list().remove(TRASH).unwrap();
}

fn reset() {
    self::remove_drag_enter();
    let drag = document().query_selector(DRAG_SELECTOR).unwrap();
    if let Some(drag) = drag {
        drag.class_list().remove(DRAG).unwrap();
    }
}

fn delete() {
    let drag = document().query_selector(DRAG_SELECTOR).unwrap();
    if let Some(drag) = drag {
        drag.set_attribute(STYLE, DISPLAY_NONE).unwrap()
    }
    reset()
}

pub struct Trash();

impl Trash {
    pub fn new() -> Trash {
        let coord = query_selector(COORDINATE_SELECTOR);
        let cries = CRIES;

        coord.add_event_listener(|event: DragOverEvent| {
            event.prevent_default();
        });

        coord.add_event_listener(|event: DragEnterEvent| {
            event.prevent_default();
            let coord = query_selector(COORDINATE_SELECTOR);
            coord.class_list().add(DRAG_ENTER).unwrap();
        });

        coord.add_event_listener(|event: DragLeaveEvent| {
            event.prevent_default();
            let coord = query_selector(COORDINATE_SELECTOR);
            coord.class_list().remove(DRAG_ENTER).unwrap();
        });

        coord.add_event_listener(enclose!((cries) move |event: DragDropEvent| {
            event.prevent_default();
            let index = get_range(0 as f64, cries.len() as f64) as usize;
            let okay = confirm(&cries[index].to_string());
            if okay {
                self::delete()
            } else {
                self::reset()
            }
        }));

        fn bind_link(link: Node) {
            let el: HtmlElement = link.clone().try_into().unwrap();
            let drag_event = |event: DragEvent| {
                let cursor = query_selector(CURSOR_SELECTOR);
                let x = f64::from(event.client_x());
                let y = f64::from(event.client_y());
                cursor
                    .set_attribute(
                        STYLE,
                        &format!("transform: translate3d({}px,{}px,0);", x, y),
                    )
                    .unwrap();
            };

            link.add_event_listener(enclose!( (el) move |_event: DragStartEvent| {
              let coord = query_selector(COORDINATE_SELECTOR);
              coord.class_list().add(TRASH).unwrap();
              let cursor = query_selector(CURSOR_SELECTOR);
              cursor.class_list().remove(ZOOM).unwrap();
              let clone = el.clone_node(CloneKind::Deep).unwrap();
              el.class_list().add(DRAG).unwrap();
              cursor.append_child(&clone);
            }));

            link.add_event_listener(drag_event);

            link.add_event_listener(|_event: DragEndEvent| {
                js! {
                  /* TODO: fix this hack! */
                  var reset = @{reset};
                  window.setTimeout(reset, 100);
                }
                self::remove_drag_enter();
                let cursor = query_selector(CURSOR_SELECTOR);
                let project = query_selector(CURSOR_PROJECT_SELECTOR);
                cursor.remove_child(&project).unwrap();
            });
        }

        for link in node_list(LINK_SELECTOR) {
            bind_link(link)
        }

        Trash()
    }
}
