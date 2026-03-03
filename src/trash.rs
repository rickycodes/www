use stdweb::web::event::{
    DragDropEvent, DragEndEvent, DragEnterEvent, DragEvent, DragLeaveEvent, DragOverEvent,
    DragStartEvent,
};

use stdweb::traits::*;
use stdweb::web::{confirm, document, CloneKind};
use crate::util::{get_range, node_list, query_selector, try_query_selector};

use stdweb::unstable::TryInto;
use stdweb::web::{HtmlElement, Node};

use crate::constants::{
    COORDINATE_SELECTOR, CRIES, CURSOR_SELECTOR, DISPLAY_NONE, DRAG, DRAG_ENTER, DRAG_SELECTOR,
    LINK_SELECTOR, STYLE, TRASH, ZOOM,
};

fn remove_drag_enter() {
    let coord = query_selector(COORDINATE_SELECTOR);
    let _ = coord.class_list().remove(DRAG_ENTER);
    let _ = coord.class_list().remove(TRASH);
}

fn reset() {
    remove_drag_enter();
    if let Ok(Some(drag)) = document().query_selector(DRAG_SELECTOR) {
        let _ = drag.class_list().remove(DRAG);
    }
}

fn delete() {
    if let Ok(Some(drag)) = document().query_selector(DRAG_SELECTOR) {
        let _ = drag.set_attribute(STYLE, DISPLAY_NONE);
    }
    reset();
}

pub(crate) struct Trash;

impl Trash {
    pub(crate) fn new() -> Self {
        let coord = query_selector(COORDINATE_SELECTOR);
        let cries = CRIES;
        let coord_for_enter = coord.clone();
        let coord_for_leave = coord.clone();

        coord.add_event_listener(|event: DragOverEvent| {
            event.prevent_default();
        });

        coord.add_event_listener(enclose!((coord_for_enter) move |event: DragEnterEvent| {
            event.prevent_default();
            let _ = coord_for_enter.class_list().add(DRAG_ENTER);
        }));

        coord.add_event_listener(enclose!((coord_for_leave) move |event: DragLeaveEvent| {
            event.prevent_default();
            let _ = coord_for_leave.class_list().remove(DRAG_ENTER);
        }));

        coord.add_event_listener(enclose!((cries) move |event: DragDropEvent| {
            event.prevent_default();
            let index = get_range(0.0, cries.len() as f64) as usize;
            let okay = confirm(cries[index]);
            if okay {
                delete()
            } else {
                reset()
            }
        }));

        fn bind_link(link: Node) {
            let el: HtmlElement = link.clone().try_into().unwrap();
            let cursor = query_selector(CURSOR_SELECTOR);
            let coord = query_selector(COORDINATE_SELECTOR);
            let cursor_for_drag = cursor.clone();
            let drag_event = move |event: DragEvent| {
                let x = f64::from(event.client_x());
                let y = f64::from(event.client_y());
                let _ = cursor_for_drag
                    .set_attribute(STYLE, &format!("transform: translate3d({}px,{}px,0);", x, y));
            };

            link.add_event_listener(enclose!((el, coord, cursor) move |_event: DragStartEvent| {
              let _ = coord.class_list().add(TRASH);
              let _ = cursor.class_list().remove(ZOOM);
              let clone = el.clone_node(CloneKind::Deep).unwrap();
              let _ = el.class_list().add(DRAG);
              let _ = cursor.append_child(&clone);
            }));

            link.add_event_listener(drag_event);

            link.add_event_listener(|_event: DragEndEvent| {
                js! {
                  /* TODO: fix this hack! */
                  var reset = @{reset};
                  window.setTimeout(reset, 100);
                }
                remove_drag_enter();
                if let Some(cursor) = try_query_selector(CURSOR_SELECTOR) {
                    if let Ok(Some(project)) = cursor.query_selector(".project") {
                        let _ = cursor.remove_child(&project);
                    }
                }
            });
        }

        for link in node_list(LINK_SELECTOR) {
            bind_link(link)
        }

        Self
    }
}
