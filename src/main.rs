#[macro_use]
extern crate stdweb;

extern crate rand;

#[macro_use]
mod enclose;
mod coordinates;
mod cursors;
mod links;
mod slideshows;
mod toggle_project;
mod trash;
mod util;
mod work_history;
mod constants;

use coordinates::Coordinates;
use cursors::Cursors;
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
        Coordinates::new();
        Cursors::new();
        Trash::new();
        WorkHistory::new();
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
