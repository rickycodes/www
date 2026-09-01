use crate::util::{get_hash, query_selector};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{HashChangeEvent, KeyDownEvent};
use stdweb::web::{document, window, Element, HtmlElement};

use crate::constants::{
    ACTIVE, ACTIVE_PROJECT_SELECTOR, CLASS, DATA_PROJECT, DATA_SCROLL, EMPTY, INERT,
    PROJECT_SELECTOR,
};

const TAB: &str = "Tab";
const ACTIVE_DIALOG_SELECTOR: &str = "[data-project] .project.is-active";
const FOCUSABLE_SELECTOR: &str = "a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex=\"-1\"])";

fn set_active_project(project: Option<Element>) {
    if let Some(active_project) = document().query_selector(ACTIVE_PROJECT_SELECTOR).unwrap() {
        let class = active_project
            .get_attribute(CLASS)
            .unwrap_or_default()
            .split_whitespace()
            .filter(|class| *class != ACTIVE)
            .collect::<Vec<_>>()
            .join(" ");
        active_project.set_attribute(CLASS, &class).unwrap();
        active_project.set_attribute(INERT, EMPTY).unwrap();
    }

    if let Some(project) = project {
        let class = project.get_attribute(CLASS).unwrap_or_default();
        project
            .set_attribute(CLASS, &format!("{} {}", class, ACTIVE))
            .unwrap();
        project.remove_attribute(INERT);
        let project: HtmlElement = project.try_into().unwrap();
        project.focus();
    }
}

fn show(hash: String, scroll_top: &mut Option<f64>, return_focus: &mut Option<HtmlElement>) {
    let body = document().body().unwrap();
    let selector = &format!(".projects .project.{}", hash);
    if let Some(project) = document().query_selector(selector).unwrap() {
        if let Some(opener) = document()
            .query_selector(&format!("._projects .project.link.{}", hash))
            .unwrap()
        {
            *return_focus = opener.try_into().ok();
        }
        set_active_project(Some(project));
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

fn trap_focus(shift: bool) {
    let dialog = match document().query_selector(ACTIVE_DIALOG_SELECTOR).unwrap() {
        Some(dialog) => dialog,
        None => return,
    };

    let focusable: Vec<HtmlElement> = dialog
        .query_selector_all(FOCUSABLE_SELECTOR)
        .unwrap()
        .into_iter()
        .filter_map(|node| node.try_into().ok())
        .collect();
    if focusable.is_empty() {
        return;
    }

    let focused_index = dialog
        .query_selector(":focus")
        .unwrap()
        .and_then(|focused| {
            focusable
                .iter()
                .position(|element| focused.as_ref() == element.as_ref())
        });
    let current_index = focused_index.unwrap_or(if shift { 0 } else { focusable.len() - 1 });
    let target_index = if shift {
        (current_index + focusable.len() - 1) % focusable.len()
    } else {
        (current_index + 1) % focusable.len()
    };

    focusable[target_index].focus();
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

        let keydown_event = move |event: KeyDownEvent| {
            if get_hash() != EMPTY && event.key() == TAB {
                event.prevent_default();
                trap_focus(event.shift_key());
            }
        };
        window().add_event_listener(keydown_event);

        Self
    }
}
