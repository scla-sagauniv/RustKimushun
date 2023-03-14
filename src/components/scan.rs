use crate::components::{back_button::BackButton, shutter_button::ShutterButton};
use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html, use_effect_with_deps};

#[function_component(Scan)]
pub fn scan() -> Html {
  fn test() {

  #[wasm_bindgen(module="/src/js/script.js")]
    extern "C"{
        fn displayCaps();
    }

  #[wasm_bindgen]
    pub fn temp (){
      displayCaps()
    }
    temp();
  }
  use_effect_with_deps(move |_| {
      wasm_bindgen_futures::spawn_local(async move {
          test();
      });
      || ()
  }, ());
  
  html! {
    // プロパティの値は、リテラルか中括弧で囲む必要があります。式を囲む中括弧の追加を検討してください。
      <>
        <div class="container p-0">
          <div class="position-absolute" style="left: 17px; margin-top: 25px; top: 18px;">
            <BackButton title={""} destination={"/"}/>
          </div>
          <div  class="d-flex flex-column align-items-center">
            <div class="position-absolute top-0 mt-5">
              <h5 class="text-white">{"QRコードを読み取ってください"}</h5>
            </div>
            <div class="position-absolute bottom-0 mb-5">
              <ShutterButton title={""} destination={"#"}/>
            </div>
            </div>
            <video id="camera" width="375" height="667" style="z-index: -1;" class="w-100 h-100 position-absolute top-0"></video>
            <canvas id="picture" width="375" height="667" style="z-index: -1;" class="w-100 h-100 position-absolute top-0"></canvas>
        </div>
      </>
    }
}