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
            class: "bg-black text-white py-2 px-4 rounded-md hover:bg-black/90",
            {props.children}
        }
    }
} 