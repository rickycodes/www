use stdweb::traits::*;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{document, window};
use util::{get_hash, qs};

fn toggle(scrolls: &mut Vec<f64>) {
    let hash = get_hash();
    let body = document().body().unwrap();
    if hash != "" {
        let selector = &format!(".projects .project.{}", hash);
        if document().query_selector(selector).unwrap().is_some() {
            let top = Some(window().page_y_offset())
                .unwrap_or_else(|| document().document_element().unwrap().scroll_top());
            body.set_attribute("data-project", &hash).unwrap();
            scrolls.push(top);
            qs("[data-project] > .content").set_scroll_top(top)
        }
    } else {
        body.remove_attribute("data-project");
        let top = *scrolls.last().unwrap_or(&0.00);
        document().document_element().unwrap().set_scroll_top(top);
        document().body().unwrap().set_scroll_top(top);
        body.remove_attribute("data-scroll")
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
