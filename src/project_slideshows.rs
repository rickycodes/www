use crate::util::{create_element, node_list};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, KeyUpEvent};
use stdweb::web::{document, window, HtmlElement, Node};

use crate::constants::{
    ACTIVE, ARIA_LABEL, ARIA_PRESSED, ARROW_LEFT, ARROW_RIGHT, BUTTON, CLASS, CONTROLS, DATA_INDEX,
    DATA_PROJECT, DIV, EMPTY, ESC, LINK, NEXT, NEXT_SLIDE_ARIA_LABEL, PREV,
    PREVIOUS_SLIDE_ARIA_LABEL, SLIDE, SLIDESHOW_SELECTOR,
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
    pub(crate) fn new(slideshow_el: &HtmlElement, slides: &[HtmlElement]) -> Vec<HtmlElement> {
        let controls_el = create_element(DIV, CONTROLS);
        let mut controls = Vec::new();

        for (index, _slide) in slides.iter().enumerate() {
            let control_el = create_element(BUTTON, LINK);
            control_el.set_text_content(&(index + 1).to_string());
            control_el
                .set_attribute(ARIA_LABEL, &format!("Show slide {}", index + 1))
                .unwrap();
            control_el
                .set_attribute(ARIA_PRESSED, if index == 0 { "true" } else { "false" })
                .unwrap();
            controls.push(control_el.clone());
            controls_el.append_child(&control_el);
        }

        for (index, control) in controls.iter().enumerate() {
            let all_slides = slides.to_vec();
            let all_controls = controls.clone();
            control.add_event_listener(
                enclose!( (slideshow_el, index, all_slides, all_controls) move |event:ClickEvent| {
                    event.prevent_default();
                    slideshow_el.set_attribute(DATA_INDEX, &index.to_string()).unwrap();
                    set_active(&all_slides, &all_controls, index);
                }),
            );
        }

        slideshow_el
            .parent_node()
            .unwrap()
            .append_child(&controls_el);

        controls
    }
}

fn set_active(slides: &[HtmlElement], controls: &[HtmlElement], index: usize) {
    for (slide_index, slide) in slides.iter().enumerate() {
        let class = if slide_index == index {
            format!("{} {}", SLIDE, ACTIVE)
        } else {
            SLIDE.to_string()
        };
        slide.set_attribute(CLASS, &class).unwrap();
    }

    for (control_index, control) in controls.iter().enumerate() {
        control
            .set_attribute(
                ARIA_PRESSED,
                if control_index == index {
                    "true"
                } else {
                    "false"
                },
            )
            .unwrap();
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

            let slideshow_el: HtmlElement = slideshow.try_into().unwrap();
            let controls = if slides.len() > 1 {
                Controls::new(&slideshow_el, &slides)
            } else {
                Vec::new()
            };
            set_active(&slides, &controls, 0);

            // only add navigation for slideshows with more than one slide
            if slides.len() > 1 {
                let slideshow_prev = create_element(BUTTON, PREV);
                slideshow_prev
                    .set_attribute(ARIA_LABEL, PREVIOUS_SLIDE_ARIA_LABEL)
                    .unwrap();
                slideshow_el.append_child(&slideshow_prev);

                let slideshow_next = create_element(BUTTON, NEXT);
                slideshow_next
                    .set_attribute(ARIA_LABEL, NEXT_SLIDE_ARIA_LABEL)
                    .unwrap();
                slideshow_el.append_child(&slideshow_next);

                let last = slides.len() - 1;

                let prev_next_click =
                    move |direction: &str,
                          slideshow_el: &HtmlElement,
                          slides: &[HtmlElement],
                          controls: &[HtmlElement]| {
                        let increment =
                            get_increment(direction, get_data_index(slideshow_el), last);
                        let _ = slideshow_el.set_attribute(DATA_INDEX, &increment.to_string());
                        set_active(slides, controls, increment);
                    };

                let previous_slides = slides.clone();
                let previous_controls = controls.clone();
                let slideshow_prev_event = enclose!( (slideshow_el, previous_slides, previous_controls) move |event: ClickEvent| {
                    event.prevent_default();
                    prev_next_click(PREV, &slideshow_el, &previous_slides, &previous_controls)
                });

                let next_slides = slides.clone();
                let next_controls = controls.clone();
                let slideshow_next_event = enclose!( (slideshow_el, next_slides, next_controls) move |event: ClickEvent| {
                    event.prevent_default();
                    prev_next_click(NEXT, &slideshow_el, &next_slides, &next_controls)
                });

                slideshow_prev.add_event_listener(slideshow_prev_event);
                slideshow_next.add_event_listener(slideshow_next_event);
            }
        }

        // use keyboard to navigate
        let next_prev_click = |selector: &str| {
            if let Ok(Some(element)) = document().query_selector(selector) {
                js!( @{element}.click(); );
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
