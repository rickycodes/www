#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(unreachable_pub, future_incompatible, rust_2018_idioms)]

#[macro_use]
extern crate stdweb;

extern crate rand;

#[macro_use]
mod enclose;
mod constants;
mod coordinates;
mod cursors;
mod links;
mod project_slideshows;
mod project;
mod trash;
mod util;
mod work_history;

use crate::coordinates::Coordinates;
use crate::cursors::Cursors;
use crate::links::Links;
use crate::project_slideshows::SlideShows;
use crate::project::ToggleProject;
use crate::trash::Trash;
use crate::util::set_date;
use crate::work_history::WorkHistory;
use crate::constants::log;

struct Website;

impl Website {
    fn set_date(self) -> Self {
        set_date();
        self
    }

    fn initialize(self) -> Self {
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

    fn event_loop(self) -> Self {
        stdweb::event_loop();
        self
    }

    fn new() -> Self {
        log();
        Self
    }
}

fn main() {
    // welcome to my homepage i kiss you
    Website::new().initialize().set_date().event_loop();
}
