use stdweb::traits::*;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{document, window};
use util::{get_hash, qs};

use constants::{
    DATA_PROJECT,
    DATA_SCROLL
};

fn toggle(scrolls: &mut Vec<f64>) {
    let hash = get_hash();
    let body = document().body().unwrap();
    let document_element = document().document_element();
    if hash != "" {
        let selector = &format!(".projects .project.{}", hash);
        let query = document().query_selector(selector).unwrap();
        if query.is_some() {
            let top = Some(window().page_y_offset())
                .unwrap_or_else(|| document_element.unwrap().scroll_top());
            body.set_attribute(DATA_PROJECT, &hash).unwrap();
            scrolls.push(top);
            qs("[data-project] > .content").set_scroll_top(top)
        }
    } else {
        body.remove_attribute(DATA_PROJECT);
        let top = *scrolls.last().unwrap_or(&0.00);
        document_element.unwrap().set_scroll_top(top);
        body.set_scroll_top(top);
        body.remove_attribute(DATA_SCROLL)
    }
}

pub struct ToggleProject();

impl ToggleProject {
    pub fn new() -> ToggleProject {
        let mut scrolls = Vec::new();
        toggle(&mut scrolls);
        let toggle_project_event = move |_event: HashChangeEvent| toggle(&mut scrolls);
        window().add_event_listener(toggle_project_event);

        ToggleProject()
    }
}
