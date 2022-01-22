use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, KeyUpEvent};
use stdweb::web::{document, window, HtmlElement, Node};
use crate::util::{create_element, node_list};

use crate::constants::{
    A, ARROW_LEFT, ARROW_RIGHT, CONTROLS, DATA_INDEX, DATA_PROJECT, DIV, EMPTY, ESC, LINK, NEXT,
    PREV, SLIDESHOW_SELECTOR, UNDERSCORE,
};

fn get_data_index(element: &HtmlElement) -> usize {
    element.get_attribute(DATA_INDEX).unwrap().parse().unwrap()
}

fn set_data_index_attribute(element: &HtmlElement, attribute: &str) {
    element.set_attribute(DATA_INDEX, attribute).unwrap();
}

fn get_increment(direction: &str, data_index: usize, last: usize) -> usize {
    let increment = if direction == PREV {
        if data_index == 0 {
            last
        } else {
            data_index - 1
        }
    } else {
        if data_index == last {
            0
        } else {
            data_index + 1
        }
    };

    increment
}

pub(crate) struct Controls();

impl Controls {
    pub(crate) fn new(slideshow_el: &HtmlElement, slides: &Vec<HtmlElement>) -> Controls {
        let controls_el = create_element(DIV, CONTROLS);

        let control_setup = |index: usize| {
            let control_el = create_element(A, LINK);
            control_el.set_text_content(&(index + 1).to_string());
            control_el.add_event_listener(enclose!( (slideshow_el, index) move |event:ClickEvent| {
              event.prevent_default();
              slideshow_el.set_attribute(DATA_INDEX, &index.to_string()).unwrap();
            }));
            controls_el.append_child(&control_el);
        };

        for (index, _slide) in slides.iter().enumerate() {
            control_setup(index)
        }

        slideshow_el
            .parent_node()
            .unwrap()
            .append_child(&controls_el);

        Controls()
    }
}

pub(crate) struct Slide();

impl Slide {
    pub(crate) fn new(node: Node) -> HtmlElement {
        node.try_into().unwrap()
    }
}

pub(crate) struct SlideShows();

impl SlideShows {
    pub(crate) fn new() -> SlideShows {
        // setup all slideshows
        for slideshow in node_list(SLIDESHOW_SELECTOR) {
            // collect slides
            let slides: Vec<HtmlElement> = slideshow
                .child_nodes()
                .into_iter()
                .filter(|item| item.node_name() == DIV.to_uppercase())
                .map(Slide::new)
                .collect();

            // only setup slideshow if there is more than one slide!
            if slides.len() > 1 {
                let slideshow_el: HtmlElement = slideshow.try_into().unwrap();

                let slideshow_prev = create_element(A, PREV);
                slideshow_el.append_child(&slideshow_prev);

                let slideshow_next = create_element(A, NEXT);
                slideshow_el.append_child(&slideshow_next);

                Controls::new(&slideshow_el, &slides);

                let prev_next_click =
                    |direction: &str, slideshow_el: &HtmlElement, slides: &Vec<HtmlElement>| {
                        let increment = self::get_increment(
                            direction,
                            self::get_data_index(&slideshow_el),
                            slides.len() - 1,
                        );
                        self::set_data_index_attribute(&slideshow_el, &increment.to_string())
                    };

                let slideshow_prev_event = enclose!( (slideshow_el, slides) move |_: ClickEvent| {
                    prev_next_click(&PREV, &slideshow_el, &slides)
                });

                let slideshow_next_event = enclose!( (slideshow_el, slides) move |_: ClickEvent| {
                    prev_next_click(&NEXT, &slideshow_el, &slides)
                });

                slideshow_prev.add_event_listener(slideshow_prev_event);
                slideshow_next.add_event_listener(slideshow_next_event);
            }
        }

        // use keyboard to navigate
        let next_prev_click = |selector: &str| {
            if document().query_selector(selector).unwrap().is_some() {
                js!( document.querySelector(@{selector}).click(); );
            }
        };

        let determine_key = |key: String| match key.as_ref() {
            ARROW_LEFT => PREV,
            ARROW_RIGHT => NEXT,
            _ => UNDERSCORE,
        };

        let keyup_event = move |event: KeyUpEvent| {
            let data_project = document().body().unwrap().get_attribute(DATA_PROJECT);
            if data_project.is_some() {
                let key = event.key();
                if key == ESC {
                    js!( window.location.hash = @{EMPTY}; );
                } else {
                    let next_prev_key = determine_key(key);
                    if next_prev_key != UNDERSCORE {
                        let selector =
                            &format!(".project.{} .{}", data_project.unwrap(), next_prev_key);
                        next_prev_click(selector)
                    }
                }
            }
        };

        window().add_event_listener(keyup_event);

        SlideShows()
    }
}
