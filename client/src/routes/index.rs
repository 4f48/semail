use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "h-[100%] w-[100%] flex items-center justify-center gap-3",
            Link {
                to: Route::Login {},
                "Login"
            },
            Link {
                to: Route::Register {},
                "Register"
            }
        }
    }
}