use yew::{classes, function_component, html, Html};

#[function_component(App)]
fn app() -> Html {

    html! {
        <>
        // 背景
        <div class={classes!("flex", "flex-col", "min-h-screen", "justify-between", "text-center", "bg-sky-400")}>
            <div class={classes!("bg-sky-900")}>
                <p class={classes!("py-5")}>{"Chisenon.work"}</p>
            </div>
            
            // メインコンテンツ
            <div class={classes!()}>
                <h1 class={classes!("text-white", "text-[72px]", "font-bold", "mx-auto")}>
                    {"のんのんチャレンジ"}
                </h1>

                <p class={classes!("text-white", "text-2xl", "flex", "justify-center")}>
                    {"ランダムな「平仮名２文字 + のん 」が10個生成されます！"}
                </p>
            </div>
            
            <div class={classes!()}>
                <a href="https://x.com/home" target="_blank" class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-10", "px-20", "border", "border-blue-700", "rounded", "text-[24px]")}>
                    {"ついーのんする"}
                </a>
                <p class={classes!("text-white", "text-1xl", "flex", "justify-center")}>
                    {"※機能未実装※"}
                </p>
            </div>
            

            <div class={classes!()}>
                <a href="https://www.google.com" target="_blank" class={classes!("text-white", "text-[24px]", "font-bold", "mx-auto")}>
                    {"セフィラちゃんはいいぞ"}
                </a>
            </div>

            // フッター
            <div class={classes!("bg-sky-900")}>
                <h1 class={classes!("chisenon_color", "py-3")}>{"Made By Chisenon"}</h1>
            </div>            
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}