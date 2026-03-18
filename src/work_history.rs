use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, Element};

use crate::constants::{EMPTY, HASH, WORK_HISTORY};
use crate::util::get_hash;

fn scroll_into_view(el: Element) {
    js! { @(no_return)
        let el = @{el};
        setTimeout(() => {
            el.scrollIntoView();
        }, 10);
    }
}

pub(crate) struct WorkHistory;

impl WorkHistory {
    fn open_from_hash(details: Element) {
        if get_hash() == WORK_HISTORY {
            let details_for_js = details.clone();
            js! { @(no_return)
                let details = @{details_for_js};
                details.open = true;
            }
            self::scroll_into_view(details);
        }
    }

    pub(crate) fn new() -> Self {
        let selector = format!(".{}", WORK_HISTORY);
        let details = document().query_selector(&selector).unwrap().unwrap();
        Self::open_from_hash(details.clone());
        let click_event = enclose!( (details) move |_: ClickEvent| {
            let clone = details.clone();
            let is_open: bool = js!( return @{&details}.open; )
                .try_into()
                .unwrap();

            let hash = if is_open {
                EMPTY.to_string()
            } else {
                format!("{}{}", HASH, WORK_HISTORY)
            };

            js! { @(no_return)
                window.location.hash = @{hash};
            }

            if !is_open {
                self::scroll_into_view(clone)
            }
        });
        details.add_event_listener(click_event);
        Self
    }
}
