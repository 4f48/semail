use dioxus::prelude::*;
use crate::components::link::Lin;
use crate::Route;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "h-[100%] w-[100%] flex items-center justify-center gap-3",
            Lin {
                href: "/login",
                children: rsx! {
                    "Login"
                }
            }
            Lin {
                href: "/register",
                children: rsx! {
                    "Register"
                }
            }
        }
    }
}