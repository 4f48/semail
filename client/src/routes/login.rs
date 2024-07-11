use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Login() -> Element {
    rsx! {
        h1 {
            class: "text-red",
            "Login"
        }
    }
}