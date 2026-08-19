use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::event::ClickEvent;
use stdweb::web::{document, window, HtmlElement};
use stdweb::Mut;
use stdweb::{js, traits::*, unstable::TryInto, Reference};

use crate::constants::{THEME_DARK, THEME_LIGHT, THEME_SELECTOR, THEME_STORAGE_KEY};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ThemePreference {
    System,
    Light,
    Dark,
}

impl ThemePreference {
    fn from_storage(value: Option<String>) -> Self {
        match value {
            Some(value) => {
                if value == THEME_DARK {
                    Self::Dark
                } else if value == THEME_LIGHT {
                    Self::Light
                } else {
                    Self::System
                }
            }
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

fn system_prefers_dark() -> bool {
    js!( return window.matchMedia("(prefers-color-scheme: dark)").matches; )
        .try_into()
        .unwrap_or(false)
}

fn set_theme(button: &HtmlElement, dark: bool) {
    let body = document().body().unwrap();
    let theme = if dark { THEME_DARK } else { THEME_LIGHT };
    let label = if dark {
        "Switch to light mode"
    } else {
        "Switch to dark mode"
    };
    let icon = if dark { "🌞" } else { "🌚" };

    body.set_attribute("data-theme", theme).unwrap();
    button.set_attribute("aria-label", label).unwrap();
    button
        .set_attribute("aria-pressed", if dark { "true" } else { "false" })
        .unwrap();
    button.set_text_content(icon);
}

fn watch_system_preference(button: HtmlElement, state: Rc<RefCell<ThemeState>>) {
    let callback = move |_event: Reference| {
        let follows_system = state.borrow().preference == ThemePreference::System;
        if follows_system {
            let dark = system_prefers_dark();
            state.borrow_mut().dark = dark;
            set_theme(&button, dark);
        }
    };

    js! {
        var mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
        var listener = @{Mut(callback)};
        if (mediaQuery.addEventListener) {
            mediaQuery.addEventListener("change", listener);
        } else {
            mediaQuery.addListener(listener);
        }
    }
}

pub(crate) struct Theme;

impl Theme {
    pub(crate) fn new() -> Self {
        let button: HtmlElement = document()
            .query_selector(THEME_SELECTOR)
            .unwrap()
            .expect("theme toggle")
            .try_into()
            .unwrap();
        let storage = window().local_storage();
        let preference = ThemePreference::from_storage(storage.get(THEME_STORAGE_KEY));
        let dark = preference.is_dark().unwrap_or_else(system_prefers_dark);
        let state = Rc::new(RefCell::new(ThemeState { preference, dark }));

        set_theme(&button, dark);
        watch_system_preference(button.clone(), state.clone());

        let event_button = button.clone();
        let event_storage = storage.clone();
        button.add_event_listener(move |_event: ClickEvent| {
            let mut state = state.borrow_mut();
            state.dark = !state.dark;
            state.preference = if state.dark {
                ThemePreference::Dark
            } else {
                ThemePreference::Light
            };
            set_theme(&event_button, state.dark);
            event_button.blur();
            let _ = event_storage.insert(
                THEME_STORAGE_KEY,
                if state.dark { THEME_DARK } else { THEME_LIGHT },
            );
        });

        Self
    }
}
