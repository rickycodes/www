use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, Element};
use util::get_hash;

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
            js!{
                const details = @{clone};
                const scroll_into_view = @{scroll_into_view};
                if (!details.open) {
                    scroll_into_view(details);
                }
            };
        });
        details.add_event_listener(click_event);
        WorkHistory()
    }
}
