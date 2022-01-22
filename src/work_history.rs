use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, Element};
use stdweb::web::event::ClickEvent;

use crate::constants::WORK_HISTORY_SELECTOR;

fn scroll_into_view(el: Element) {
    js! { @(no_return)
        let el = @{el};
        setTimeout(() => {
            el.scrollIntoView();
        }, 10);
    }
}

pub(crate) struct WorkHistory();

impl WorkHistory {
    pub(crate) fn new() -> WorkHistory {
        let details = document()
            .query_selector(WORK_HISTORY_SELECTOR)
            .unwrap()
            .unwrap();
        let click_event = enclose!( (details) move |_: ClickEvent| {
            let clone = details.clone();
            let is_open: bool = js!( return @{&details}.open; )
                .try_into()
                .unwrap();

            if !is_open {
                self::scroll_into_view(clone)
            }
        });
        details.add_event_listener(click_event);
        WorkHistory()
    }
}
