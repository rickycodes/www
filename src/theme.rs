use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, window, HtmlElement};

use crate::constants::{THEME_DARK, THEME_LIGHT, THEME_SELECTOR, THEME_STORAGE_KEY};

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
        let dark = match storage.get(THEME_STORAGE_KEY) {
            Some(value) => {
                if value == THEME_DARK {
                    true
                } else if value == THEME_LIGHT {
                    false
                } else {
                    system_prefers_dark()
                }
            }
            None => system_prefers_dark(),
        };

        set_theme(&button, dark);

        let event_button = button.clone();
        let event_storage = storage.clone();
        button.add_event_listener(move |_event: ClickEvent| {
            let dark = document().body().unwrap().get_attribute("data-theme")
                == Some(THEME_DARK.to_string());
            let next = !dark;
            set_theme(&event_button, next);
            let _ = event_storage.insert(
                THEME_STORAGE_KEY,
                if next { THEME_DARK } else { THEME_LIGHT },
            );
        });

        Self
    }
}
