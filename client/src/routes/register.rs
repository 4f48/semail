use crate::components::input::Input;
use crate::components::submit::Submit;
use crate::Route;

use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Register() -> Element {
    async fn handle_submit(event: FormEvent) {
        let values = event.data.values();
        info!("{values:?}");
    }

    rsx! {
        h1 {
            class: "text-2xl font-bold",
            "Create your account"
        }
        div {
            class: "flex flex-col gap-2 w-96",
            form {
                class: "flex flex-col gap-2 w-full",
                onsubmit: handle_submit,
                Input {
                    name: "Username",
                    input_type: "text"
                }
                Input {
                    name: "Password",
                    input_type: "password"
                }
                Submit {
                    value: "Register"
                }
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
