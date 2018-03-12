use util::{qs,get_hash};
use stdweb::traits::*;
use stdweb::web::{
  document,
  window
};

pub fn toggle() {
  let hash = get_hash();
  let body = document().body().unwrap();
  if hash != "" {
    let selector = &format!(".projects .project.{}", hash);
    if document().query_selector(selector).unwrap().is_some() {
      let top = Some(window().page_y_offset())
        .unwrap_or(document().document_element().unwrap().scroll_top());
      body.set_attribute("data-project", &hash).unwrap();
      body.set_attribute("data-scroll", &top.to_string()).unwrap();
      qs("[data-project] > .content").set_scroll_top(top as u32)
    }
  } else {
    body.remove_attribute("data-project");
    let top = body.get_attribute("data-scroll").unwrap_or("0".to_string());
    let top_as_num = top.parse::<u32>().unwrap();
    document().document_element().unwrap().set_scroll_top(top_as_num);
    body.remove_attribute("data-scroll")
  }
}
