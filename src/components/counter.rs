use yew::{function_component, html, Html};
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter +1;
            counter.set(value);
        }
    };
    
    html! {
        <div class={classes!()}>
            <button {onclick} class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-3", "px-3", "border", "border-blue-700", "rounded", "text-[24px]")}>
                {"+1"}
            </button>
            <p>{*counter}</p>
        </div>
    }
}