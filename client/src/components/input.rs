use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct Props {
    name: String,
    input_type: String,
}

#[component]
pub fn Input(props: Props) -> Element {
    rsx! {
        input {
            class: "border border-gray-300 shadow-md transition-colors h-10 px-4 py-2 w-full rounded-md text-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black",
            name: "{props.name.to_lowercase()}",
            placeholder: "{props.name}",
            r#type: "{props.input_type}"
        }
    }
}
