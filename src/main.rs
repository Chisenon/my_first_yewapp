use yew::{classes, function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        
        <div class={classes!("bg-indigo-300", "text-white", "w-screen")}>
            
            <h1 class={classes!("text-center", "justify-center", "items-center", "text-[min(13vw,100px)]")}>
                {"こんにちわぁ！！！"}
            </h1>

            <img class={classes!()} src="https://static.miraheze.org/bluearchivewiki/3/34/Reisa.png" alt="reisa" />
            
            <p>{"これ、3個目くらいの環境だったりする。"}</p>
            <p>{"ゆるせん..."}</p>
            <ul>
                <li>{"戦術対抗戦がやばい"}</li>
                <li>{"水ハナコが強すぎる"}</li>
                <li>{"シュン持ってないぞ"}</li>
            </ul>
        
        </div>
        
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}