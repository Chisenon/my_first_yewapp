use yew::{classes, function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        //大枠のdiv
        // <div class={classes!("bg-indigo-300", "text-white", "w-screen")}>
            
        //     <h1 class={classes!("text-center", "justify-center", "items-center", "text-[min(13vw,100px)]")}>
        //         {"こんにちわぁ！！！"}
        //     </h1>

        //     <img class={classes!()} src="https://static.miraheze.org/bluearchivewiki/3/34/Reisa.png" alt="reisa" />
            
        //     <p>{"これ、3個目くらいの環境だったりする。"}</p>
        //     <p>{"ゆるせん..."}</p>
        //     <ul>
        //         <li>{"戦術対抗戦がやばい"}</li>
        //         <li>{"水ハナコが強すぎる"}</li>
        //         <li>{"シュン持ってないぞ"}</li>
        //     </ul>
        
        // </div>

        // 背景
        <div class={classes!("h-screen", "w-2/3", "bg-sky-400", "text-center")}>
            // ヘッダー
            <div class={classes!("bg-sky-900", "mb-5")}>
                <p>{"のんのんチャレンジ"}</p>
            </div>
            
            // メインコンテンツ
            <div class={classes!()}>
                <h1 class={classes!("text-white", "text-[48px]", "font-bold", "mx-auto", )}>
                    {"のんのんチャレンジ"}
                </h1>
            </div>

            // フッター
            <div class={classes!("bg-sky-900")}>

            </div>
            // <p class={classes!("text-2xl", "flex", "justify-center")}>{"ランダムな平仮名２文字 + のん が10個生成されます！"}</p>

            
        </div>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}