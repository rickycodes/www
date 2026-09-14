use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

use crate::constants::{
    ACTIVE, ARIA_LABEL, ARIA_PRESSED, ARROW_LEFT, ARROW_RIGHT, BUTTON, CLASS, CONTROLS, DATA_INDEX,
    DATA_PROJECT, DIV, EMPTY, ESC, LINK, NEXT, NEXT_SLIDE_ARIA_LABEL, PREV,
    PREVIOUS_SLIDE_ARIA_LABEL, SLIDE, SLIDESHOW_SELECTOR,
};
use crate::util::{create_element, document, listen, node_list, window};

fn get_data_index(element: &HtmlElement) -> usize {
    element
        .get_attribute(DATA_INDEX)
        .expect("slideshow index")
        .parse()
        .expect("numeric slideshow index")
}

fn get_increment(direction: &str, data_index: usize, last: usize) -> usize {
    let len = last + 1;
    if direction == PREV {
        (data_index + last) % len
    } else {
        (data_index + 1) % len
    }
}

fn set_active(slides: &[HtmlElement], controls: &[HtmlElement], index: usize) {
    for (slide_index, slide) in slides.iter().enumerate() {
        let class = if slide_index == index {
            format!("{SLIDE} {ACTIVE}")
        } else {
            SLIDE.to_string()
        };
        slide.set_attribute(CLASS, &class).expect("slide class");
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
            .expect("control state");
    }
}

struct Controls;

impl Controls {
    fn build(slideshow: &HtmlElement, slides: &[HtmlElement]) -> Vec<HtmlElement> {
        let controls_element = create_element(DIV, CONTROLS);
        let mut controls = Vec::new();

        for index in 0..slides.len() {
            let control = create_element(BUTTON, LINK);
            control.set_text_content(Some(&(index + 1).to_string()));
            control
                .set_attribute(ARIA_LABEL, &format!("Show slide {}", index + 1))
                .expect("control label");
            control
                .set_attribute(ARIA_PRESSED, if index == 0 { "true" } else { "false" })
                .expect("control state");
            controls.push(control.clone());
            controls_element
                .append_child(&control)
                .expect("slideshow control");
        }

        for (index, control) in controls.iter().enumerate() {
            let slideshow = slideshow.clone();
            let slides = slides.to_vec();
            let controls = controls.clone();
            listen(control.as_ref(), "click", move |event: MouseEvent| {
                event.prevent_default();
                slideshow
                    .set_attribute(DATA_INDEX, &index.to_string())
                    .expect("slideshow index");
                set_active(&slides, &controls, index);
            });
        }

        slideshow
            .parent_node()
            .expect("slideshow parent")
            .append_child(&controls_element)
            .expect("slideshow controls");

        controls
    }
}

fn bind_direction(
    control: &HtmlElement,
    direction: &'static str,
    slideshow: &HtmlElement,
    slides: &[HtmlElement],
    controls: &[HtmlElement],
) {
    let slideshow = slideshow.clone();
    let slides = slides.to_vec();
    let controls = controls.to_vec();
    let last = slides.len() - 1;
    listen(control.as_ref(), "click", move |event: MouseEvent| {
        event.prevent_default();
        let increment = get_increment(direction, get_data_index(&slideshow), last);
        slideshow
            .set_attribute(DATA_INDEX, &increment.to_string())
            .expect("slideshow index");
        set_active(&slides, &controls, increment);
    });
}

pub(crate) struct SlideShows;

impl SlideShows {
    pub(crate) fn new() -> Self {
        for slideshow in node_list(SLIDESHOW_SELECTOR) {
            let slideshow: HtmlElement = slideshow.dyn_into().expect("slideshow");
            let children = slideshow.child_nodes();
            let slides: Vec<HtmlElement> = (0..children.length())
                .filter_map(|index| children.item(index))
                .filter(|node| node.node_name() == "DIV")
                .filter_map(|node| node.dyn_into().ok())
                .collect();

            let controls = if slides.len() > 1 {
                Controls::build(&slideshow, &slides)
            } else {
                Vec::new()
            };
            set_active(&slides, &controls, 0);

            if slides.len() > 1 {
                let previous = create_element(BUTTON, PREV);
                previous
                    .set_attribute(ARIA_LABEL, PREVIOUS_SLIDE_ARIA_LABEL)
                    .expect("previous label");
                slideshow.append_child(&previous).expect("previous control");

                let next = create_element(BUTTON, NEXT);
                next.set_attribute(ARIA_LABEL, NEXT_SLIDE_ARIA_LABEL)
                    .expect("next label");
                slideshow.append_child(&next).expect("next control");

                bind_direction(&previous, PREV, &slideshow, &slides, &controls);
                bind_direction(&next, NEXT, &slideshow, &slides, &controls);
            }
        }

        listen(window().as_ref(), "keyup", move |event: KeyboardEvent| {
            let Some(body) = document().body() else {
                return;
            };
            let Some(project) = body.get_attribute(DATA_PROJECT) else {
                return;
            };

            match event.key().as_str() {
                ESC => window()
                    .location()
                    .set_hash(EMPTY)
                    .expect("clear project hash"),
                ARROW_LEFT => click_direction(&project, PREV),
                ARROW_RIGHT => click_direction(&project, NEXT),
                _ => {}
            }
        });

        Self
    }
}

fn click_direction(project: &str, direction: &str) {
    let selector = format!(".project.{project} .{direction}");
    if let Some(element) = document()
        .query_selector(&selector)
        .expect("slideshow direction selector")
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
    {
        element.click();
    }
}
