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

use links::Links;
use slideshows::SlideShows;
use toggle_project::ToggleProject;
use trash::Trash;
use util::set_date;
use work_history::WorkHistory;
use canvas::Canvas;

#[derive(Debug, Clone, Copy)]
struct Website();

impl Website {
    fn new() -> Website {
        stdweb::initialize();

        SlideShows::new();
        Links::new();
        Canvas::new();
        Trash::new();
        WorkHistory::new();
        ToggleProject::new();

        set_date();

        stdweb::event_loop();

        Website()
    }
}

fn main() {
    Website::new();
}
