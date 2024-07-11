use dioxus::prelude::*;
use crate::components::link::Lin;
use crate::Route;

#[component]
pub fn Index() -> Element {
    rsx! {
        h1 {
            class: "text-2xl font-bold",
            "SE-Mail web panel"
        }
        div {
            class: "flex flex-col gap-2 w-96",
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