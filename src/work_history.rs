use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, Element};

use stdweb::web::event::ClickEvent;

fn scroll_into_view(el: Element) {
    js! { @(no_return)
      @{el}.scrollIntoView();
    }
}

pub struct WorkHistory();

impl WorkHistory {
    pub fn new() -> WorkHistory {
        let details = document().query_selector(".work-history").unwrap().unwrap();
        let click_event = enclose!( (details) move |_: ClickEvent| {
            let clone = details.clone();
            let is_open: bool = js!( return @{&details}.open; )
                .try_into()
                .unwrap();

            if !is_open {
                scroll_into_view(clone)
            }
        });
        details.add_event_listener(click_event);
        WorkHistory()
    }
}
