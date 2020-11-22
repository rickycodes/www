use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, KeyUpEvent};
use stdweb::web::{document, window, HtmlElement};
use util::{create_el, nl};

use constants::{DATA_INDEX, DATA_PROJECT, ESC, NEXT, PREV};

macro_rules! prev_next {
    ($slideshow:expr, $slides:expr, $dir:expr) => {
        let last = $slides.len() - 1;
        let data_index: usize = $slideshow
            .get_attribute(DATA_INDEX)
            .unwrap()
            .parse()
            .unwrap();
        let inc: usize = if $dir == NEXT {
            if data_index == last {
                0
            } else {
                data_index + 1
            }
        } else {
            if data_index == 0 {
                last
            } else {
                data_index - 1
            }
        };
        $slideshow
            .set_attribute(DATA_INDEX, &inc.to_string())
            .unwrap();
    };
}

pub struct SlideShows();

impl SlideShows {
    pub fn new() -> SlideShows {
        // setup all slideshows
        for slideshow in nl(".slideshow") {
            let mut slides = Vec::new();

            // collect slides
            for child in slideshow.child_nodes() {
                if child.node_name() == "DIV" {
                    let el: HtmlElement = child.try_into().unwrap();
                    slides.push(el);
                }
            }

            if slides.len() > 1 {
                // only setup slideshow if there is more than one slide!
                let slideshow_el: HtmlElement = slideshow.try_into().unwrap();

                let slideshow_prev = create_el("a", PREV);
                slideshow_el.append_child(&slideshow_prev);

                let slideshow_next = create_el("a", NEXT);
                slideshow_el.append_child(&slideshow_next);

                let controls_el = create_el("div", "controls");

                let control_setup = |index: usize| {
                    let control_el = create_el("a", "link");
                    control_el.set_text_content(&(index + 1).to_string());
                    control_el.add_event_listener(
                        enclose!( (slideshow_el, index) move |_:ClickEvent| {
                          slideshow_el.set_attribute(DATA_INDEX, &index.to_string()).unwrap();
                        }),
                    );
                    controls_el.append_child(&control_el);
                };

                for (index, _slide) in slides.iter().enumerate() {
                    control_setup(index)
                }

                slideshow_el
                    .parent_node()
                    .unwrap()
                    .append_child(&controls_el);

                let slideshow_prev_event = enclose!( (slideshow_el, slides) move |_: ClickEvent| {
                  prev_next!(slideshow_el, slides, PREV);
                });

                let slideshow_next_event = enclose!( (slideshow_el, slides) move |_: ClickEvent| {
                  prev_next!(slideshow_el, slides, NEXT);
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
            "ArrowLeft" => PREV,
            "ArrowRight" => NEXT,
            _ => "_",
        };

        let keyup_event = move |event: KeyUpEvent| {
            let data_project = document().body().unwrap().get_attribute(DATA_PROJECT);
            if data_project.is_some() {
                let key = event.key();
                if key == ESC {
                    js!( window.location.hash = ""; );
                } else {
                    let next_prev = determine_key(key);
                    if next_prev != "_" {
                        let selector =
                            &format!(".project.{} .{}", data_project.unwrap(), next_prev);
                        next_prev_click(selector)
                    }
                }
            }
        };

        window().add_event_listener(keyup_event);

        SlideShows()
    }
}
