#[macro_use]
extern crate stdweb;

#[macro_use]
mod enclose;
mod canvas;
mod links;
mod slideshows;
mod toggle_project;
mod trash;
mod util;
mod work_history;

use stdweb::traits::*;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{window, Date};
use toggle_project::toggle;
use util::qs;
use work_history::{bind_work_toggle, scroll_to};

#[derive(Debug, Clone, Copy)]
pub struct Website();

impl Website {
    fn new() -> Website {
        stdweb::initialize();
        slideshows::initialize();
        links::initialize();
        canvas::initialize();
        trash::initialize();

        let mut scrolls = Vec::new();

        qs(".date").set_text_content(&Date::new().get_full_year().to_string());
        toggle(&mut scrolls);
        scroll_to();
        bind_work_toggle();
        let toggle_project_event = move |_event: HashChangeEvent| toggle(&mut scrolls);

        window().add_event_listener(toggle_project_event);

        stdweb::event_loop();

        Website()
    }
}

fn main() {
    Website::new();
}
