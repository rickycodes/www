use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use crate::constants::{
    ACTIVE, ACTIVE_PROJECT_SELECTOR, DATA_PROJECT, DATA_SCROLL, EMPTY, INERT, PROJECT_SELECTOR,
};
use crate::util::{document, get_hash, listen, query_selector, window};

const TAB: &str = "Tab";
const ACTIVE_DIALOG_SELECTOR: &str = "[data-project] .project.is-active";
const FOCUSABLE_SELECTOR: &str = "a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex=\"-1\"])";

fn set_active_project(project: Option<Element>) {
    if let Some(active_project) = document()
        .query_selector(ACTIVE_PROJECT_SELECTOR)
        .expect("active project selector")
    {
        active_project
            .class_list()
            .remove_1(ACTIVE)
            .expect("active class");
        active_project.set_attribute(INERT, EMPTY).expect("inert");
    }

    if let Some(project) = project {
        project.class_list().add_1(ACTIVE).expect("active class");
        project.remove_attribute(INERT).expect("remove inert");
        let project: HtmlElement = project.dyn_into().expect("project HTML element");
        project.focus().expect("focus project");
    }
}

fn show(hash: String, scroll_top: &mut Option<i32>, return_focus: &mut Option<HtmlElement>) {
    let body = document().body().expect("body");
    let selector = format!(".projects .project.{hash}");
    if let Some(project) = document()
        .query_selector(&selector)
        .expect("project selector")
    {
        if let Some(opener) = document()
            .query_selector(&format!("._projects .project.link.{hash}"))
            .expect("project opener selector")
        {
            *return_focus = opener.dyn_into().ok();
        }
        set_active_project(Some(project));
        let top = window().page_y_offset().unwrap_or_default() as i32;
        body.set_attribute(DATA_PROJECT, &hash)
            .expect("project state");
        *scroll_top = Some(top);
        query_selector(PROJECT_SELECTOR).set_scroll_top(top);
    }
}

fn hide(scroll_top: &mut Option<i32>, return_focus: &mut Option<HtmlElement>) {
    let body = document().body().expect("body");
    set_active_project(None);
    let top = scroll_top.take().unwrap_or_default();
    body.remove_attribute(DATA_PROJECT).expect("project state");
    if let Some(document_element) = document().document_element() {
        let document_element: HtmlElement =
            document_element.dyn_into().expect("document HTML element");
        document_element.set_scroll_top(top);
    }
    body.set_scroll_top(top);
    body.remove_attribute(DATA_SCROLL).expect("scroll state");
    if let Some(opener) = return_focus.take() {
        opener.focus().expect("restore focus");
    }
}

fn toggle(scroll_top: &mut Option<i32>, return_focus: &mut Option<HtmlElement>) {
    let hash = get_hash();
    if hash != EMPTY {
        show(hash, scroll_top, return_focus);
    } else {
        hide(scroll_top, return_focus);
    }
}

fn trap_focus(shift: bool) {
    let Some(dialog) = document()
        .query_selector(ACTIVE_DIALOG_SELECTOR)
        .expect("dialog selector")
    else {
        return;
    };

    let nodes = dialog
        .query_selector_all(FOCUSABLE_SELECTOR)
        .expect("focusable selector");
    let focusable: Vec<HtmlElement> = (0..nodes.length())
        .filter_map(|index| nodes.item(index))
        .filter_map(|node| node.dyn_into().ok())
        .collect();
    if focusable.is_empty() {
        return;
    }

    let active = document().active_element();
    let focused_index = active.and_then(|focused| {
        focusable
            .iter()
            .position(|element| element.is_same_node(Some(&focused)))
    });
    let current_index = focused_index.unwrap_or(if shift { 0 } else { focusable.len() - 1 });
    let target_index = if shift {
        (current_index + focusable.len() - 1) % focusable.len()
    } else {
        (current_index + 1) % focusable.len()
    };

    focusable[target_index]
        .focus()
        .expect("focus dialog control");
}

pub(crate) struct ToggleProject;

impl ToggleProject {
    pub(crate) fn new() -> Self {
        let mut scroll_top = None;
        let mut return_focus = None;
        toggle(&mut scroll_top, &mut return_focus);
        listen(
            window().as_ref(),
            "hashchange",
            move |_event: web_sys::Event| {
                toggle(&mut scroll_top, &mut return_focus);
            },
        );

        listen(window().as_ref(), "keydown", move |event: KeyboardEvent| {
            if get_hash() != EMPTY && event.key() == TAB {
                event.prevent_default();
                trap_focus(event.shift_key());
            }
        });

        Self
    }
}
