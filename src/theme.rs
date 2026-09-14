use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MediaQueryListEvent, MouseEvent, Storage};

use crate::constants::{THEME_DARK, THEME_LIGHT, THEME_SELECTOR, THEME_STORAGE_KEY};
use crate::util::{document, listen, window};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ThemePreference {
    System,
    Light,
    Dark,
}

impl ThemePreference {
    fn from_storage(value: Option<String>) -> Self {
        match value.as_deref() {
            Some(THEME_DARK) => Self::Dark,
            Some(THEME_LIGHT) => Self::Light,
            _ => Self::System,
        }
    }

    fn is_dark(self) -> Option<bool> {
        match self {
            Self::System => None,
            Self::Light => Some(false),
            Self::Dark => Some(true),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ThemeState {
    preference: ThemePreference,
    dark: bool,
}

fn system_preference() -> Option<web_sys::MediaQueryList> {
    window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
}

fn system_prefers_dark() -> bool {
    system_preference()
        .map(|query| query.matches())
        .unwrap_or(false)
}

fn set_theme(button: &HtmlElement, dark: bool) {
    let document_element = document().document_element().expect("document element");
    let theme = if dark { THEME_DARK } else { THEME_LIGHT };
    let label = if dark {
        "Switch to light mode"
    } else {
        "Switch to dark mode"
    };
    let icon = if dark { "🌚" } else { "🌞" };

    document_element
        .set_attribute("data-theme", theme)
        .expect("theme");
    button
        .set_attribute("aria-label", label)
        .expect("theme label");
    button
        .set_attribute("aria-pressed", if dark { "true" } else { "false" })
        .expect("theme state");
    button.set_text_content(Some(icon));
}

fn watch_system_preference(button: HtmlElement, state: Rc<RefCell<ThemeState>>) {
    let Some(media_query) = system_preference() else {
        return;
    };
    listen(
        media_query.as_ref(),
        "change",
        move |event: MediaQueryListEvent| {
            if state.borrow().preference == ThemePreference::System {
                let dark = event.matches();
                state.borrow_mut().dark = dark;
                set_theme(&button, dark);
            }
        },
    );
}

fn local_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

pub(crate) struct Theme;

impl Theme {
    pub(crate) fn new() -> Self {
        let button: HtmlElement = document()
            .query_selector(THEME_SELECTOR)
            .expect("theme selector")
            .expect("theme toggle")
            .dyn_into()
            .expect("theme button");
        let storage = local_storage();
        let stored = storage
            .as_ref()
            .and_then(|storage| storage.get_item(THEME_STORAGE_KEY).ok().flatten());
        let preference = ThemePreference::from_storage(stored);
        let dark = preference.is_dark().unwrap_or_else(system_prefers_dark);
        let state = Rc::new(RefCell::new(ThemeState { preference, dark }));

        set_theme(&button, dark);
        watch_system_preference(button.clone(), state.clone());

        let event_button = button.clone();
        listen(button.as_ref(), "click", move |_event: MouseEvent| {
            let mut state = state.borrow_mut();
            state.dark = !state.dark;
            state.preference = if state.dark {
                ThemePreference::Dark
            } else {
                ThemePreference::Light
            };
            set_theme(&event_button, state.dark);
            event_button.blur().expect("blur theme button");
            if let Some(storage) = &storage {
                let _ = storage.set_item(
                    THEME_STORAGE_KEY,
                    if state.dark { THEME_DARK } else { THEME_LIGHT },
                );
            }
        });

        Self
    }
}
