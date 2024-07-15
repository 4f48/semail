#![allow(non_snake_case)]

mod components;
mod layouts;
mod routes;

use layouts::form::Layout;
use routes::index::Index;
use routes::login::Login;
use routes::register::Register;

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Index {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
    const _CSS_URL: &str = manganis::mg!(file("assets/main.css"));
    const _INTER: &str = manganis::mg!(font().families(["Inter"]));

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
