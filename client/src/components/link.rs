use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct Props {
    href: String,
    children: Element
}

#[component]
pub fn Lin(props: Props) -> Element {
    rsx! {
        Link {
            to: "{props.href}",
            class: "bg-black text-white hover:bg-black/90 h-10 px-4 py-2 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
            {props.children}
        }
    }
} 