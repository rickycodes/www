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

use canvas::Canvas;
use links::Links;
use slideshows::SlideShows;
use toggle_project::ToggleProject;
use trash::Trash;
use util::set_date;
use work_history::WorkHistory;

#[derive(Debug, Clone, Copy)]
struct Website();

impl Website {
    pub fn set_date(self) -> Self {
        set_date();
        self
    }

    pub fn initialize(self) -> Self {
        stdweb::initialize();

        SlideShows::new();
        Links::new();
        Canvas::new();
        Trash::new();
        WorkHistory::new().scroll_to().bind_work_toggle();
        ToggleProject::new();
        self
    }

    pub fn event_loop(self) -> Self {
        stdweb::event_loop();
        self
    }

    fn new() -> Website {
        Website()
    }
}

fn main() {
    Website::new().initialize().set_date().event_loop();
}
