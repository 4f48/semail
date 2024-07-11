use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Register() -> Element {
    rsx! {
        h1 {
            "Register"
        }
    }
}