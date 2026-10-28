use js_sys::{Array, Date, Intl, JSON, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use web_sys::{Event, HtmlElement, HtmlImageElement, IntersectionObserver, XmlHttpRequest};

use crate::constants::{
    BLANK, HASH, HREF, NOOPENER, PROJECT_ITEM_SELECTOR, PROJECT_LINK_SELECTOR, REL, TARGET, TITLE,
};
use crate::util::{document, elements_from_nodes, listen, node_list, window};

const BUILD_META_SELECTOR: &str = ".build-meta";
const COMMIT_BASE_URL: &str = "https://github.com/rickycodes/www/commit/";
const BUILD_META_PREFIX: &str = "Deployed: ";

pub(crate) struct Page;

impl Page {
    pub(crate) fn initialize() {
        render_build_meta();
        initialize_images();
    }
}

fn render_build_meta() {
    let Some(element) = document()
        .query_selector(BUILD_META_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let element: HtmlElement = element.unchecked_into();

    let Ok(request) = XmlHttpRequest::new() else {
        render_unknown(&element);
        return;
    };
    if request
        .open_with_async("GET", "build-meta.json", true)
        .is_err()
    {
        render_unknown(&element);
        return;
    }
    let _ = request.set_request_header("Cache-Control", "no-store");

    let target = element.clone();
    let callback_request = request.clone();
    let onload = move |_| {
        let result = callback_request
            .response_text()
            .ok()
            .flatten()
            .and_then(|json| {
                JSON::parse(&json).ok().map(|meta| {
                    let built_at = property_string(&meta, "built_at_utc");
                    let git_sha = property_string(&meta, "git_sha");
                    build_meta_view(built_at.as_deref(), git_sha.as_deref(), format_shipped_at)
                })
            });
        match result {
            Some(view) if callback_request.status().unwrap_or_default() == 200 => {
                render_meta(&target, view);
            }
            _ => render_unknown(&target),
        }
    };
    let target: &web_sys::EventTarget = request.as_ref();
    listen::<Event, _>(target, "load", onload);
    let target = element.clone();
    for event_type in ["error", "abort", "timeout"] {
        listen::<Event, _>(request.as_ref(), event_type, {
            let target = target.clone();
            move |_| render_unknown(&target)
        });
    }
    if request.send().is_err() {
        render_unknown(&element);
    }
}

fn property_string(value: &JsValue, key: &str) -> Option<String> {
    Reflect::get(value, &JsValue::from_str(key))
        .ok()?
        .as_string()
        .filter(|value| !value.is_empty())
}

fn render_unknown(element: &HtmlElement) {
    element.set_text_content(Some(&format!("{BUILD_META_PREFIX}unknown")));
}

#[derive(Debug, PartialEq)]
struct BuildMetaView {
    built_at: Option<String>,
    git_sha: Option<String>,
}

fn build_meta_view(
    built_at: Option<&str>,
    git_sha: Option<&str>,
    format_date: impl Fn(&str) -> Option<String>,
) -> BuildMetaView {
    let built_at = built_at
        .filter(|value| !value.is_empty())
        .map(|value| format_date(value).unwrap_or_else(|| value.to_owned()));
    let git_sha = git_sha.filter(|value| !value.is_empty()).map(str::to_owned);
    BuildMetaView { built_at, git_sha }
}

fn render_meta(element: &HtmlElement, view: BuildMetaView) {
    element.set_text_content(None);
    element
        .append_child(&document().create_text_node(BUILD_META_PREFIX).into())
        .expect("append build metadata");

    if let Some(built_at) = view.built_at.as_deref() {
        element
            .append_child(&document().create_text_node(built_at).into())
            .expect("append build date");
    } else if view.git_sha.is_none() {
        element
            .append_child(&document().create_text_node("unknown").into())
            .expect("append unknown build metadata");
    }

    if let Some(sha) = view.git_sha.as_deref() {
        if view.built_at.is_some() {
            element
                .append_child(&document().create_text_node(",").into())
                .expect("append build separator");
            element
                .append_child(&document().create_element("br").expect("line break"))
                .expect("append line break");
        }
        let link = document().create_element("a").expect("commit link");
        link.set_attribute(HREF, &format!("{COMMIT_BASE_URL}{sha}"))
            .expect("commit URL");
        link.set_attribute(TITLE, "View this commit on GitHub")
            .expect("commit title");
        link.set_attribute(TARGET, BLANK).expect("commit target");
        link.set_attribute(REL, NOOPENER).expect("commit rel");
        link.set_text_content(Some(sha));
        element.append_child(&link).expect("append commit link");
    }

    if view.built_at.is_some() || view.git_sha.is_some() {
        element
            .append_child(&document().create_text_node(".").into())
            .expect("append build metadata period");
    }
}

fn format_shipped_at(value: &str) -> Option<String> {
    let date = Date::new(&JsValue::from_str(value));
    if date.get_time().is_nan() {
        return None;
    }
    let options = Object::new();
    for (key, value) in [
        ("day", "numeric"),
        ("month", "short"),
        ("year", "numeric"),
        ("timeZone", "America/Toronto"),
    ] {
        Reflect::set(&options, &JsValue::from_str(key), &JsValue::from_str(value)).ok()?;
    }
    let date_formatter =
        Intl::DateTimeFormat::new(&Array::of1(&JsValue::from_str("en-CA")), &options);
    let date_parts = date_formatter.format_to_parts(&date);
    let part = |kind: &str| {
        date_parts.iter().find_map(|part| {
            let kind_matches = Reflect::get(&part, &JsValue::from_str("type"))
                .ok()
                .and_then(|value| value.as_string())
                .is_some_and(|value| value == kind);
            kind_matches
                .then(|| {
                    Reflect::get(&part, &JsValue::from_str("value"))
                        .ok()
                        .and_then(|value| value.as_string())
                })
                .flatten()
        })
    };
    let month = part("month")?;
    let day: u32 = part("day")?.parse().ok()?;
    let year = part("year")?;
    let date_part = format!("{month} {day}{}, {year}", ordinal_suffix(day));
    let time_options = Object::new();
    for (key, value) in [("timeStyle", "short"), ("timeZone", "America/Toronto")] {
        Reflect::set(
            &time_options,
            &JsValue::from_str(key),
            &JsValue::from_str(value),
        )
        .ok()?;
    }
    let time = date
        .to_locale_time_string_with_options("en-CA", &time_options)
        .as_string()?;
    Some(format!("{date_part}, {time} ET"))
}

fn ordinal_suffix(day: u32) -> &'static str {
    match day % 100 {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildMetaView, build_meta_view, ordinal_suffix, project_name_from_hash};

    fn test_view(built_at: Option<&str>, git_sha: Option<&str>) -> BuildMetaView {
        build_meta_view(built_at, git_sha, |value| {
            (value != "invalid").then(|| format!("formatted {value}"))
        })
    }

    #[test]
    fn ordinal_suffixes_cover_regular_and_teen_days() {
        for (day, expected) in [
            (0, "th"),
            (1, "st"),
            (2, "nd"),
            (3, "rd"),
            (4, "th"),
            (11, "th"),
            (12, "th"),
            (13, "th"),
            (21, "st"),
            (22, "nd"),
            (23, "rd"),
            (111, "th"),
            (112, "th"),
            (113, "th"),
        ] {
            assert_eq!(ordinal_suffix(day), expected, "day {day}");
        }
    }

    #[test]
    fn project_hash_parser_returns_nonempty_fragment_names() {
        assert_eq!(project_name_from_hash("#metamask"), Some("metamask"));
        assert_eq!(
            project_name_from_hash("#audio-visualizer"),
            Some("audio-visualizer")
        );
    }

    #[test]
    fn project_hash_parser_rejects_missing_or_empty_fragments() {
        assert_eq!(project_name_from_hash(""), None);
        assert_eq!(project_name_from_hash("metamask"), None);
        assert_eq!(project_name_from_hash("#"), None);
    }

    #[test]
    fn build_metadata_with_no_fields_renders_unknown_state() {
        assert_eq!(
            test_view(None, None),
            BuildMetaView {
                built_at: None,
                git_sha: None,
            }
        );
        assert_eq!(
            test_view(Some(""), Some("")),
            BuildMetaView {
                built_at: None,
                git_sha: None,
            }
        );
    }

    #[test]
    fn build_metadata_keeps_valid_date_and_commit_fields() {
        assert_eq!(
            test_view(Some("iso-date"), Some("abc123")),
            BuildMetaView {
                built_at: Some("formatted iso-date".into()),
                git_sha: Some("abc123".into()),
            }
        );
    }

    #[test]
    fn missing_date_or_commit_keeps_the_available_field() {
        assert_eq!(
            test_view(None, Some("abc123")),
            BuildMetaView {
                built_at: None,
                git_sha: Some("abc123".into()),
            }
        );
        assert_eq!(
            test_view(Some("iso-date"), None),
            BuildMetaView {
                built_at: Some("formatted iso-date".into()),
                git_sha: None,
            }
        );
    }

    #[test]
    fn invalid_date_is_preserved_while_missing_date_is_omitted() {
        assert_eq!(
            test_view(Some("invalid"), Some("abc123")),
            BuildMetaView {
                built_at: Some("invalid".into()),
                git_sha: Some("abc123".into()),
            }
        );
        assert_eq!(
            test_view(None, Some("abc123")),
            BuildMetaView {
                built_at: None,
                git_sha: Some("abc123".into()),
            }
        );
    }
}

fn initialize_images() {
    let lazy_images: Vec<HtmlImageElement> = node_list("img[data-src]")
        .into_iter()
        .filter_map(|element| element.dyn_into::<HtmlImageElement>().ok())
        .filter(|image| {
            image
                .closest(PROJECT_ITEM_SELECTOR)
                .ok()
                .flatten()
                .is_none()
        })
        .collect();

    if let Ok(observer) = make_image_observer() {
        for image in lazy_images {
            observer.observe(image.unchecked_ref());
        }
    } else {
        for image in lazy_images {
            load_image(&image);
        }
    }

    for link in node_list(PROJECT_LINK_SELECTOR) {
        let Ok(link) = link.dyn_into::<HtmlElement>() else {
            continue;
        };
        let opener = link.clone();
        listen::<Event, _>(link.as_ref(), "pointerenter", move |_| {
            preload_from_link(&opener)
        });
        let opener = link.clone();
        listen::<Event, _>(link.as_ref(), "focus", move |_| preload_from_link(&opener));
    }

    listen::<Event, _>(window().as_ref(), "hashchange", |_| preload_open_project());
    preload_open_project();
}

fn make_image_observer() -> Result<IntersectionObserver, JsValue> {
    let callback = Closure::<dyn FnMut(Array, IntersectionObserver)>::new(
        move |entries: Array, observer: IntersectionObserver| {
            for entry in entries.iter() {
                let Some(target) = Reflect::get(&entry, &JsValue::from_str(TARGET))
                    .ok()
                    .and_then(|target| target.dyn_into::<HtmlImageElement>().ok())
                else {
                    continue;
                };
                let is_intersecting = Reflect::get(&entry, &JsValue::from_str("isIntersecting"))
                    .ok()
                    .and_then(|value| value.as_bool())
                    .unwrap_or(false);
                if is_intersecting {
                    load_image(&target);
                    observer.unobserve(target.unchecked_ref());
                }
            }
        },
    );
    let observer = IntersectionObserver::new(callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(observer)
}

fn load_image(image: &HtmlImageElement) {
    if let Some(source) = image.get_attribute("data-src") {
        image.set_src(&source);
        let _ = image.remove_attribute("data-src");
    }
}

fn preload_from_link(link: &HtmlElement) {
    if let Some(hash) = link.get_attribute(HREF) {
        preload_project(&hash);
    }
}

fn preload_open_project() {
    if let Ok(hash) = window().location().hash() {
        preload_project(&hash);
    }
}

fn project_name_from_hash(hash: &str) -> Option<&str> {
    hash.strip_prefix(HASH).filter(|name| !name.is_empty())
}

fn preload_project(hash: &str) {
    let Some(name) = project_name_from_hash(hash) else {
        return;
    };
    let Ok(Some(project)) = document().query_selector(&format!("{PROJECT_ITEM_SELECTOR}.{name}"))
    else {
        return;
    };
    let images = project
        .query_selector_all("img[data-src]")
        .expect("project image selector");
    for image in elements_from_nodes(images)
        .into_iter()
        .filter_map(|element| element.dyn_into::<HtmlImageElement>().ok())
    {
        load_image(&image);
    }
}
