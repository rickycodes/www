use util::{qs,get_hash};
use stdweb::traits::*;
use stdweb::web::{
  document,
  window
};

pub fn toggle(scrolls: &mut Vec<f64>) {
  let hash = get_hash();
  let body = document().body().unwrap();
  if hash != "" {
    let selector = &format!(".projects .project.{}", hash);
    if document().query_selector(selector).unwrap().is_some() {
      let top = Some(window().page_y_offset())
        .unwrap_or(document().document_element().unwrap().scroll_top());
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
