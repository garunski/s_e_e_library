use dioxus::prelude::*;

use crate::library_href;
use crate::routes::route_for_path;

#[component]
pub fn AppLink(
    path: &'static str,
    #[props(default)] class: &'static str,
    #[props(default)] aria_current: &'static str,
    children: Element,
) -> Element {
    let class_attr = (!class.is_empty()).then_some(class);
    let current_attr = (!aria_current.is_empty()).then_some(aria_current);
    if let Some(route) = route_for_path(path) {
        let nav = navigator();
        let onclick = move |event: MouseEvent| {
            if !event.modifiers().is_empty() {
                return;
            }
            if event.trigger_button() != Some(dioxus::html::input_data::MouseButton::Primary) {
                return;
            }
            event.prevent_default();
            nav.push(route.clone());
        };
        rsx! {
            a {
                href: "{library_href(path)}",
                class: class_attr,
                aria_current: current_attr,
                onclick: onclick,
                {children}
            }
        }
    } else {
        rsx! {
            a {
                href: "{library_href(path)}",
                class: class_attr,
                aria_current: current_attr,
                {children}
            }
        }
    }
}
