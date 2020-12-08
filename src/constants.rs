pub const TARGET: &str = "target";
pub const BLANK: &str = "_blank";
pub const REL: &str = "rel";
pub const NOOPENER: &str = "noopener";
pub const CLOSE: &str = "close";
pub const TRASH: &str = "trash";
pub const DRAG: &str = "drag";
pub const STYLE: &str = "style";
pub const ZOOM: &str = "zoom";
pub const NEXT: &str = "next";
pub const PREV: &str = "prev";
pub const ESC: &str = "Escape";
pub const DATA_INDEX: &str = "data-index";
pub const DATA_PROJECT: &str = "data-project";
pub const DATA_SCROLL: &str = "data-scroll";
pub const HIDDEN: &str = "hidden";
pub const NAME: &str = "name";
pub const TITLE: &str = "title";
pub const EMPTY: &str = "";
pub const ZERO: &str = "0";
pub const UNDERSCORE: &str = "_";
pub const ARROW_LEFT: &str = "ArrowLeft";
pub const ARROW_RIGHT: &str = "ArrowRight";
pub const DIV: &str = "div";
pub const A: &str = "a";
pub const CONTROLS: &str = "controls";
pub const LINK: &str = "link";
pub const WORK_HISTORY_SELECTOR: &str = ".work-history";
pub const POOP: &str = "💩";
pub const CLASS: &str = "class";
pub const HASH: &str = "#";
pub const DISPLAY_NONE: &str = "display: none;";

pub const CURSOR_SELECTOR: &str = ".cursor";
pub const CURSOR_PROJECT_SELECTOR: &str = ".cursor .project";
pub const DRAG_ENTER: &str = "dragenter";
pub const DRAG_SELECTOR: &str = ".drag";
pub const COORDINATE_SELECTOR: &str = ".coord";
pub const LINK_SELECTOR: &str = ".project.link, .cv.link";
pub const CLOSE_SELECTOR: &str = ".close div";
pub const NOT_PROJECT_SELECTOR: &str = ".content a:not(.project)";
pub const PROJECT_SELECTOR: &str = "[data-project] > .content";
pub const COORDINATES_SELECTOR: &str = ".coord > div";
pub const INFO_SELECTOR: &str = ".info";
pub const INFO_LINKS_SELECTOR: &str = ".content a[title], .content label[name]";
pub const SLIDESHOW_SELECTOR: &str = ".slideshow";
pub const YEAR_SELECTOR: &str = ".year";
pub const X_SELECTOR: &str = ".x";
pub const Y_SELECTOR: &str = ".y";

pub struct CursorAttributes {
    pub selector: &'static str,
    pub classname: &'static str,
}

pub const CRIES: [&str; 5] = [
    "U sure?",
    "Really?",
    "Y u gotta be this way?",
    "Come on?",
    "Please no",
];

pub const CURSORS_ATTRIBUTES: [CursorAttributes; 5] = [
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
