pub(crate) const CRIES: [&str; 8] = [
    "U sure?",
    "Really?",
    "Y u gotta be this way?",
    "Come on?",
    "Please no",
    "(ಥ﹏ಥ)",
    "༼ ༎ຶ ෴ ༎ຶ༽",
    "（>﹏<）",
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
pub(crate) const DATA_THEME: &str = "data-theme";
pub(crate) const DATA_PROJECT: &str = "data-project";
pub(crate) const DATA_SCROLL: &str = "data-scroll";
pub(crate) const HIDDEN: &str = "hidden";
pub(crate) const NAME: &str = "name";
pub(crate) const TITLE: &str = "title";
pub(crate) const EMPTY: &str = "";
pub(crate) const ARROW_LEFT: &str = "ArrowLeft";
pub(crate) const ARROW_RIGHT: &str = "ArrowRight";
pub(crate) const MOUSEOVER: &str = "mouseover";
pub(crate) const MOUSEOUT: &str = "mouseout";
pub(crate) const DIV: &str = "div";
pub(crate) const A: &str = "a";
pub(crate) const BUTTON: &str = "button";
pub(crate) const TYPE: &str = "type";
pub(crate) const CONTROLS: &str = "controls";
pub(crate) const LINK: &str = "link";
pub(crate) const WORK_HISTORY: &str = "work-history";
pub(crate) const POOP: &str = "💩";
pub(crate) const CLASS: &str = "class";
pub(crate) const HASH: &str = "#";
pub(crate) const DISPLAY_NONE: &str = "display: none;";
pub(crate) const HREF: &str = "href";
pub(crate) const ARIA_LABEL: &str = "aria-label";
pub(crate) const ARIA_PRESSED: &str = "aria-pressed";
pub(crate) const INERT: &str = "inert";
pub(crate) const PREVIOUS_SLIDE_ARIA_LABEL: &str = "Previous slide";
pub(crate) const NEXT_SLIDE_ARIA_LABEL: &str = "Next slide";
pub(crate) const THEME_DARK: &str = "dark";
pub(crate) const THEME_LIGHT: &str = "light";
pub(crate) const THEME_STORAGE_KEY: &str = "ricky.codes.theme";
pub(crate) const THEME_SELECTOR: &str = ".theme-toggle";

pub(crate) const CURSOR_SELECTOR: &str = ".cursor";
pub(crate) const DRAG_ENTER: &str = "dragenter";
pub(crate) const DRAG_SELECTOR: &str = ".drag";
pub(crate) const COORDINATE_SELECTOR: &str = ".coord";
pub(crate) const PROJECT_LINK_SELECTOR: &str = "._projects .project.link";
pub(crate) const PROJECT_ITEM_SELECTOR: &str = ".projects .project";
pub(crate) const LINK_SELECTOR: &str = ".project.link, .cv.link";
pub(crate) const CLOSE_SELECTOR: &str = ".close div";
pub(crate) const NOT_PROJECT_SELECTOR: &str = ".content a:not(.project)";
pub(crate) const PROJECT_SELECTOR: &str = "[data-project] > .content";
pub(crate) const ACTIVE_PROJECT_SELECTOR: &str = ".projects .project.is-active";
pub(crate) const COORDINATES_SELECTOR: &str = ".coord > div";
pub(crate) const INFO_SELECTOR: &str = ".info";
pub(crate) const INFO_LINKS_SELECTOR: &str = ".content a[title], .content label[name]";
pub(crate) const SLIDESHOW_SELECTOR: &str = ".slideshow";
pub(crate) const SLIDE: &str = "slide";
pub(crate) const ACTIVE: &str = "is-active";
pub(crate) const YEAR_SELECTOR: &str = ".year";

// Shared panic context for invariant DOM operations.
pub(crate) const EXPECT_ACTIVE_CLASS: &str = "active class";
pub(crate) const EXPECT_BODY: &str = "body";
pub(crate) const EXPECT_CONTROL_STATE: &str = "control state";
pub(crate) const EXPECT_CURSOR_CLASS: &str = "cursor class";
pub(crate) const EXPECT_HTML_ELEMENT: &str = "HTML element";
pub(crate) const EXPECT_INFO_CLASS: &str = "info class";
pub(crate) const EXPECT_PROJECT_STATE: &str = "project state";
pub(crate) const EXPECT_SELECTOR: &str = "valid selector";
pub(crate) const EXPECT_SLIDESHOW_INDEX: &str = "slideshow index";

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

const CONSOLE_STYLE: &str = "font-family: 'Monaco';
font-size: 4em;
padding: 20px;
color: white;
text-shadow: 0 0 0.05em #fff, 0 0 0.2em #fe05e1, 0 0 0.3em #fe05e1;
transform: rotate(-7deg);";

pub(crate) fn log() {
    web_sys::console::log_2(&"%c welcome to my homepage".into(), &CONSOLE_STYLE.into());
    web_sys::console::log_1(
        &"The original Magic Kitty™ is an oracle that lets anyone seek advice about their future!"
            .into(),
    );
    web_sys::console::log_1(
        &"All you have to do is “ask()” any yes or no question and your answer will be revealed."
            .into(),
    );
    web_sys::console::log_1(&"example: ask('will i be pretty?')".into());
}
