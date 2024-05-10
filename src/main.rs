use yew::{function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div>
                <h1>{"Hello, World!"}</h1>
                <p>{"こんにちは、世界!"}</p>
                <p>{"これは3回目くらいの環境開発です。"}</p>

                <p>{"ゆるせん..."}</p>
            
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}