use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDetailsElement, MouseEvent};

use crate::constants::{EMPTY, HASH, WORK_HISTORY};
use crate::util::{document, get_hash, listen, set_timeout, window};

fn scroll_into_view(element: Element) {
    set_timeout(move || element.scroll_into_view(), 10);
}

pub(crate) struct WorkHistory;

impl WorkHistory {
    fn open_from_hash(details: &HtmlDetailsElement) {
        if get_hash() == WORK_HISTORY {
            details.set_open(true);
            scroll_into_view(details.clone().into());
        }
    }

    pub(crate) fn new() -> Self {
        let selector = format!(".{WORK_HISTORY}");
        let details: HtmlDetailsElement = document()
            .query_selector(&selector)
            .expect("work history selector")
            .expect("work history")
            .dyn_into()
            .expect("details element");
        Self::open_from_hash(&details);

        let event_details = details.clone();
        listen(details.as_ref(), "click", move |_event: MouseEvent| {
            let is_open = event_details.open();
            let hash = if is_open {
                EMPTY.to_string()
            } else {
                format!("{HASH}{WORK_HISTORY}")
            };
            window()
                .location()
                .set_hash(&hash)
                .expect("work history hash");
            if !is_open {
                scroll_into_view(event_details.clone().into());
            }
        });

        Self
    }
}
