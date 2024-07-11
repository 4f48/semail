#![allow(non_snake_case)]

mod routes;
mod components;

use routes::index::Index;
use routes::login::Login;
use routes::register::Register;

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Index {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
    const _CSS_URL: &str = manganis::mg!(file("assets/main.css"));
    
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

