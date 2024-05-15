use yew::{classes, function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        // 背景
        <div class={classes!("flex", "flex-col", "min-h-screen", "justify-between", "text-center", "bg-sky-400")}>
            <div class={classes!("bg-sky-900")}>
                <p class={classes!("py-5")}>{"HEADER"}</p>
            </div>
            
            // メインコンテンツ
            <div class={classes!()}>
                <h1 class={classes!("text-white", "text-[72px]", "font-bold", "mx-auto")}>
                    {"工事中"}
                </h1>
            </div>
            <div class={classes!()}>
                <img src="https://pbs.twimg.com/profile_images/1595726694706745344/A2nNf9pH_400x400.jpg" class={classes!("mx-auto")}/>
            </div>
            // フッター
            <div class={classes!("bg-sky-900")}>
                <h1 class={classes!("py-3")}>{"Made By Chisenon"}</h1>
            </div>
            // <p class={classes!("text-2xl", "flex", "justify-center")}>{"ランダムな平仮名２文字 + のん が10個生成されます！"}</p>
            
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}