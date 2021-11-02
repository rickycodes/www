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
mod slideshows;
mod toggle_project;
mod trash;
mod util;
mod work_history;

use crate::coordinates::Coordinates;
use crate::cursors::Cursors;
use crate::links::Links;
use crate::slideshows::SlideShows;
use crate::toggle_project::ToggleProject;
use crate::trash::Trash;
use crate::util::set_date;
use crate::work_history::WorkHistory;

struct Website();

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

    fn new() -> Website {
        Website()
    }
}

fn main() {
    Website::new().initialize().set_date().event_loop();
    let font_style = "font-family: 'Monaco';
    font-size: 4em;
    padding: 20px;
    color: white;
    text-shadow: 0 0 0.05em #fff, 0 0 0.2em #fe05e1, 0 0 0.3em #fe05e1;
    transform: rotate(-7deg);";
    console!(log, "%c welcome to my homepage", font_style);
    console!(log, "The original Magic Kitty™ is an oracle that lets anyone seek advice about their future!");
    console!(log, "All you have to do is simply “ask() the kitty” any yes or no question, then wait for your answer to be revealed.");
    console!(log, "example: ask('will i be pretty?')")
}
