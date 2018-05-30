use util::nl;

pub fn lazy_load_images() {
  for img in nl(".slideshow-container .slideshow div img") {
    js!{
      var el = @{img};
      el.setAttribute("src", el.getAttribute("data-src"));
      el.onload = function() {
        el.removeAttribute("data-src");
      };
    }
  };
}
