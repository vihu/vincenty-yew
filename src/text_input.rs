use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SrcProps {
    pub value: String,
    pub on_change1: Callback<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DstProps {
    pub value: String,
    pub on_change2: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

/// Controlled Text Input Component
#[function_component(SrcInput)]
pub fn src_input(props: &SrcProps) -> Html {
    let SrcProps { value, on_change1 } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change1.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" {value} {oninput} />
    }
}

/// Controlled Text Input Component
#[function_component(DstInput)]
pub fn dst_input(props: &DstProps) -> Html {
    let DstProps { value, on_change2 } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change2.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" {value} {oninput} />
    }
}
