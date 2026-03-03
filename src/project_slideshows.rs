use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, KeyUpEvent};
use stdweb::web::{document, window, HtmlElement, Node};
use crate::util::{create_element, node_list};

use crate::constants::{
    A, ARIA_LABEL, ARROW_LEFT, ARROW_RIGHT, CONTROLS, DATA_INDEX, DATA_PROJECT, DIV, EMPTY, ESC,
    LINK, NEXT, NEXT_SLIDE_ARIA_LABEL, PREV, PREVIOUS_SLIDE_ARIA_LABEL, SLIDESHOW_SELECTOR,
};

fn get_data_index(element: &HtmlElement) -> usize {
    element.get_attribute(DATA_INDEX).unwrap().parse().unwrap()
}

fn get_increment(direction: &str, data_index: usize, last: usize) -> usize {
    let len = last + 1;
    if direction == PREV {
        (data_index + last) % len
    } else {
        (data_index + 1) % len
    }
}

pub(crate) struct Controls;

impl Controls {
    pub(crate) fn new(slideshow_el: &HtmlElement, slides: &[HtmlElement]) -> Self {
        let controls_el = create_element(DIV, CONTROLS);

        for (index, _slide) in slides.iter().enumerate() {
            let control_el = create_element(A, LINK);
            control_el.set_text_content(&(index + 1).to_string());
            control_el.add_event_listener(enclose!( (slideshow_el, index) move |event:ClickEvent| {
                event.prevent_default();
                slideshow_el.set_attribute(DATA_INDEX, &index.to_string()).unwrap();
            }));
            controls_el.append_child(&control_el);
        }

        slideshow_el
            .parent_node()
            .unwrap()
            .append_child(&controls_el);

        Self
    }
}

pub(crate) struct SlideShows;

impl SlideShows {
    pub(crate) fn new() -> Self {
        // setup all slideshows
        for slideshow in node_list(SLIDESHOW_SELECTOR) {
            // collect slides
            let div_tag = DIV.to_uppercase();
            let slides: Vec<HtmlElement> = slideshow
                .child_nodes()
                .into_iter()
                .filter(|item| item.node_name() == div_tag)
                .map(|node: Node| node.try_into().unwrap())
                .collect();

            // only setup slideshow if there is more than one slide!
            if slides.len() > 1 {
                let slideshow_el: HtmlElement = slideshow.try_into().unwrap();

                let slideshow_prev = create_element(A, PREV);
                slideshow_prev
                    .set_attribute(ARIA_LABEL, PREVIOUS_SLIDE_ARIA_LABEL)
                    .unwrap();
                slideshow_el.append_child(&slideshow_prev);

                let slideshow_next = create_element(A, NEXT);
                slideshow_next
                    .set_attribute(ARIA_LABEL, NEXT_SLIDE_ARIA_LABEL)
                    .unwrap();
                slideshow_el.append_child(&slideshow_next);

                Controls::new(&slideshow_el, &slides);
                let last = slides.len() - 1;

                let prev_next_click = move |direction: &str, slideshow_el: &HtmlElement| {
                    let increment = get_increment(direction, get_data_index(slideshow_el), last);
                    let _ = slideshow_el.set_attribute(DATA_INDEX, &increment.to_string());
                };

                let slideshow_prev_event = enclose!( (slideshow_el) move |event: ClickEvent| {
                    event.prevent_default();
                    prev_next_click(PREV, &slideshow_el)
                });

                let slideshow_next_event = enclose!( (slideshow_el) move |event: ClickEvent| {
                    event.prevent_default();
                    prev_next_click(NEXT, &slideshow_el)
                });

                slideshow_prev.add_event_listener(slideshow_prev_event);
                slideshow_next.add_event_listener(slideshow_next_event);
            }
        }

        // use keyboard to navigate
        let next_prev_click = |selector: &str| {
            if let Ok(Some(_el)) = document().query_selector(selector) {
                js!( document.querySelector(@{selector}).click(); );
            }
        };

        let determine_key = |key: &str| match key {
            ARROW_LEFT => Some(PREV),
            ARROW_RIGHT => Some(NEXT),
            _ => None,
        };

        let keyup_event = move |event: KeyUpEvent| {
            if let Some(body) = document().body() {
                if let Some(data_project) = body.get_attribute(DATA_PROJECT) {
                    let key = event.key();
                    if key == ESC {
                        js!( window.location.hash = @{EMPTY}; );
                    } else if let Some(next_prev_key) = determine_key(&key) {
                        let selector = &format!(".project.{} .{}", data_project, next_prev_key);
                        next_prev_click(selector);
                    }
                }
            }
        };

        window().add_event_listener(keyup_event);

        Self
    }
}
