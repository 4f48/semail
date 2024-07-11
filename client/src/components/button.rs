use leptos::*;

#[component]
pub fn Button(children: Children, href: String) -> impl IntoView {
    view! {
        <a href=href class="text-white bg-black px-4 py-2 rounded-md hover:bg-black/90 text-sm ring-offset-background focus-visible:ring-ring focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2">
            {children()}
        </a>
    }
}
