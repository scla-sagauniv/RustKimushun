use crate::components::{back_button::BackButton, shutter_button::ShutterButton};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html, Html, MouseEvent};

#[function_component(Scan)]
pub fn scan() -> Html {
    // こんな感じで呼び出す
    #[wasm_bindgen(module = "/src/logic/encode.js")]
    extern "C" {
        fn encode(path: String) -> String;
    }
    let onclick = move |_: MouseEvent| {
        encode(String::from("/cat.png"));
    };
    html! {
      <>
      <div class="container">
      <BackButton title={""} destination={"/"}/>
      <div  class="d-flex flex-column align-items-center">
        <div class="position-absolute top-0 mt-4">
          <h3>{"QRコードを読み取ってください"}</h3>
        </div>
        <div class="position-absolute bottom-0 mb-5" onclick={onclick}>
          <ShutterButton title={""} destination={"/display"}/>
        </div>
      </div>
    </div>
      </>
    }
}
