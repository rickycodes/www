use util::{nl,create_el};
use stdweb::web::event::ClickEvent;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::HtmlElement;

macro_rules! prev_next {
  ($slideshow:expr, $slides:expr, $dir:expr) => {
    let data_index: usize = $slideshow.get_attribute("data-index").unwrap().parse().unwrap();
    let inc: usize = if $dir == "next" {
      if data_index == $slides.len() - 1 { 0 } else { data_index + 1 }
    } else {
      if data_index == 0 { $slides.len() - 1 } else { data_index - 1 }
    };
    $slideshow.set_attribute("data-index", &inc.to_string()).unwrap();
  }
}

pub fn initialize() {
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

    if slides.len() > 1 { // only setup slideshow if there is more than one slide!
      let slideshow_el: HtmlElement = slideshow.try_into().unwrap();

      let slideshow_prev = create_el("a", "prev");
      slideshow_el.append_child(&slideshow_prev);

      let slideshow_next = create_el("a", "next");
      slideshow_el.append_child(&slideshow_next);

      let controls_el = create_el("div", "controls");

      for (index, _slide) in slides.iter().enumerate() {
        let control_el = create_el("a", "control-link");
        control_el.set_text_content(&(index + 1).to_string());
        control_el.add_event_listener(enclose!( (slideshow_el, index) move |_:ClickEvent| {
          slideshow_el.set_attribute("data-index", &index.to_string()).unwrap();
        }));
        controls_el.append_child(&control_el);
      }

      slideshow_el.parent_node().unwrap().append_child(&controls_el);

      let slideshow_prev_event = enclose!( (slideshow_el, slides) move |_: ClickEvent| {
        prev_next!(slideshow_el, slides, "prev");
      });

      let slideshow_next_event = enclose!( (slideshow_el, slides) move |_: ClickEvent| {
        prev_next!(slideshow_el, slides, "next");
      });

      slideshow_prev.add_event_listener(slideshow_prev_event);
      slideshow_next.add_event_listener(slideshow_next_event);
    }
  }
}
