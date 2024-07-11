use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Register() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {
        h1 {
            class: "text-2xl font-bold",
            "Create your account"
        }
        div {
            class: "flex flex-col gap-2 w-96",
            input {
                value: "{username}",
                class: "border border-gray-300 shadow-md transition-colors h-10 px-4 py-2 w-full rounded-md text-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black",
                placeholder: "Username",
                oninput: move |event| username.set(event.value())
            }
            input {
                value: "{password}",
                class: "border border-gray-300 shadow-md transition-colors h-10 px-4 py-2 w-full rounded-md text-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black",
                placeholder: "Password",
                oninput: move |event| password.set(event.value())
            }
            button {
                class: "bg-black text-white hover:bg-black/90 h-10 px-4 py-2 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
                "Register"
            }
            p {
                class: "flex gap-1 items-center text-gray-500 text-sm text-center justify-center",
                "Already have an account?"
                Link {
                    class: "text-blue-500 hover:underline",
                    to: Route::Login {},
                    "Log in"
                }
            }
        }
    }
}