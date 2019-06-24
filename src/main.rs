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
use work_history::{WorkHistory};

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

        WorkHistory::new();

        toggle(&mut scrolls);
        let toggle_project_event = move |_event: HashChangeEvent| toggle(&mut scrolls);
        window().add_event_listener(toggle_project_event);

        stdweb::event_loop();

        Website()
    }
}

fn main() {
    Website::new();
}
