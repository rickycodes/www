use crate::util::{get_hash, query_selector};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::HashChangeEvent;
use stdweb::web::{document, window, HtmlElement};

use crate::constants::{
    ACTIVE, ACTIVE_PROJECT_SELECTOR, ARIA_HIDDEN, CLASS, DATA_PROJECT, DATA_SCROLL, EMPTY,
    PROJECT_SELECTOR,
};

fn set_active_project(selector: Option<&str>) {
    if let Some(active_project) = document().query_selector(ACTIVE_PROJECT_SELECTOR).unwrap() {
        let class = active_project
            .get_attribute(CLASS)
            .unwrap_or_default()
            .split_whitespace()
            .filter(|class| *class != ACTIVE)
            .collect::<Vec<_>>()
            .join(" ");
        active_project.set_attribute(CLASS, &class).unwrap();
        active_project.set_attribute(ARIA_HIDDEN, "true").unwrap();
    }

    if let Some(selector) = selector {
        if let Some(project) = document().query_selector(selector).unwrap() {
            let class = project.get_attribute(CLASS).unwrap_or_default();
            project
                .set_attribute(CLASS, &format!("{} {}", class, ACTIVE))
                .unwrap();
            project.set_attribute(ARIA_HIDDEN, "false").unwrap();
            let project: HtmlElement = project.try_into().unwrap();
            project.focus();
        }
    }
}

fn show(hash: String, scroll_top: &mut Option<f64>, return_focus: &mut Option<HtmlElement>) {
    let body = document().body().unwrap();
    let selector = &format!(".projects .project.{}", hash);
    if document().query_selector(selector).unwrap().is_some() {
        if let Some(opener) = document()
            .query_selector(&format!("._projects .project.link.{}", hash))
            .unwrap()
        {
            *return_focus = opener.try_into().ok();
        }
        set_active_project(Some(selector));
        let top = window().page_y_offset();
        body.set_attribute(DATA_PROJECT, &hash).unwrap();
        *scroll_top = Some(top);
        query_selector(PROJECT_SELECTOR).set_scroll_top(top)
    }
}

fn hide(scroll_top: &mut Option<f64>, return_focus: &mut Option<HtmlElement>) {
    let body = document().body().unwrap();
    set_active_project(None);
    let top = scroll_top.take().unwrap_or(0.0);
    body.remove_attribute(DATA_PROJECT);
    if let Some(document_element) = document().document_element() {
        document_element.set_scroll_top(top);
    }
    body.set_scroll_top(top);
    body.remove_attribute(DATA_SCROLL);
    if let Some(opener) = return_focus.take() {
        opener.focus();
    }
}

fn toggle(scroll_top: &mut Option<f64>, return_focus: &mut Option<HtmlElement>) {
    let hash = get_hash();
    if hash != EMPTY {
        show(hash, scroll_top, return_focus)
    } else {
        hide(scroll_top, return_focus)
    }
}

pub(crate) struct ToggleProject;

impl ToggleProject {
    pub(crate) fn new() -> Self {
        let mut scroll_top = None;
        let mut return_focus = None;
        toggle(&mut scroll_top, &mut return_focus);
        let toggle_project_event =
            move |_event: HashChangeEvent| toggle(&mut scroll_top, &mut return_focus);
        window().add_event_listener(toggle_project_event);

        Self
    }
}
