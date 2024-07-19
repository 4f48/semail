use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct Props {
    value: String,
}

#[component]
pub fn Submit(props: Props) -> Element {
    rsx! {
        input {
            class: "bg-black text-white hover:bg-black/90 h-10 px-4 py-2 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
            value: "{props.value}",
            role: "button",
            r#type: "submit"
        }
    }
}
