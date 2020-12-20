pub(crate) const CRIES: [&str; 5] = [
    "U sure?",
    "Really?",
    "Y u gotta be this way?",
    "Come on?",
    "Please no",
];

pub(crate) const TARGET: &str = "target";
pub(crate) const BLANK: &str = "_blank";
pub(crate) const REL: &str = "rel";
pub(crate) const NOOPENER: &str = "noopener";
pub(crate) const CLOSE: &str = "close";
pub(crate) const TRASH: &str = "trash";
pub(crate) const DRAG: &str = "drag";
pub(crate) const STYLE: &str = "style";
pub(crate) const ZOOM: &str = "zoom";
pub(crate) const NEXT: &str = "next";
pub(crate) const PREV: &str = "prev";
pub(crate) const ESC: &str = "Escape";
pub(crate) const DATA_INDEX: &str = "data-index";
pub(crate) const DATA_PROJECT: &str = "data-project";
pub(crate) const DATA_SCROLL: &str = "data-scroll";
pub(crate) const HIDDEN: &str = "hidden";
pub(crate) const NAME: &str = "name";
pub(crate) const TITLE: &str = "title";
pub(crate) const EMPTY: &str = "";
pub(crate) const ZERO: &str = "0";
pub(crate) const UNDERSCORE: &str = "_";
pub(crate) const ARROW_LEFT: &str = "ArrowLeft";
pub(crate) const ARROW_RIGHT: &str = "ArrowRight";
pub(crate) const DIV: &str = "div";
pub(crate) const A: &str = "a";
pub(crate) const CONTROLS: &str = "controls";
pub(crate) const LINK: &str = "link";
pub(crate) const WORK_HISTORY_SELECTOR: &str = ".work-history";
pub(crate) const POOP: &str = "💩";
pub(crate) const CLASS: &str = "class";
pub(crate) const HASH: &str = "#";
pub(crate) const DISPLAY_NONE: &str = "display: none;";

pub(crate) const CURSOR_SELECTOR: &str = ".cursor";
pub(crate) const CURSOR_PROJECT_SELECTOR: &str = ".cursor .project";
pub(crate) const DRAG_ENTER: &str = "dragenter";
pub(crate) const DRAG_SELECTOR: &str = ".drag";
pub(crate) const COORDINATE_SELECTOR: &str = ".coord";
pub(crate) const LINK_SELECTOR: &str = ".project.link, .cv.link";
pub(crate) const CLOSE_SELECTOR: &str = ".close div";
pub(crate) const NOT_PROJECT_SELECTOR: &str = ".content a:not(.project)";
pub(crate) const PROJECT_SELECTOR: &str = "[data-project] > .content";
pub(crate) const COORDINATES_SELECTOR: &str = ".coord > div";
pub(crate) const INFO_SELECTOR: &str = ".info";
pub(crate) const INFO_LINKS_SELECTOR: &str = ".content a[title], .content label[name]";
pub(crate) const SLIDESHOW_SELECTOR: &str = ".slideshow";
pub(crate) const YEAR_SELECTOR: &str = ".year";
pub(crate) const X_SELECTOR: &str = ".x";
pub(crate) const Y_SELECTOR: &str = ".y";

pub(crate) struct CursorAttributes {
    pub(crate) selector: &'static str,
    pub(crate) classname: &'static str,
}

pub(crate) const CURSORS_ATTRIBUTES: [CursorAttributes; 5] = [
    CursorAttributes {
        selector: ".github.link",
        classname: "gh",
    },
    CursorAttributes {
        selector: ".twitter.link",
        classname: "tw",
    },
    CursorAttributes {
        selector: "._projects .project",
        classname: ZOOM,
    },
    CursorAttributes {
        selector: ".slideshow .prev",
        classname: PREV,
    },
    CursorAttributes {
        selector: ".slideshow .next",
        classname: NEXT,
    },
];
