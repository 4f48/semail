use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "h-[100%] w-[100%] flex flex-col gap-5 items-center justify-center",
            Outlet::<Route> {}
        }
    }
}
