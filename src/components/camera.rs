use crate::components::{back_button::BackButton, shutter_button::ShutterButton};
use yew::{function_component, html, Html, use_effect_with_deps};
use wasm_bindgen::prelude::*;

#[function_component(Camera)]
pub fn camera() -> Html {
  fn test() {

    #[wasm_bindgen(module="/src/js/script.js")]
    extern "C"{
        fn displayCaps();
    }

    #[wasm_bindgen]
    pub fn temp2 (){
      displayCaps()
    }
    temp2();
  }
  use_effect_with_deps(move |_| {
      wasm_bindgen_futures::spawn_local(async move {
          test();
      });
      || ()
  }, ());
    html! {
      <>
        <div class="container p-0">
            <div class="position-absolute" style="left: 17px; margin-top: 25px; top: 18px;">
              <BackButton title={""} destination={"/"}/>
            </div>
            <div  class="d-flex flex-column align-items-center">
              <div class="position-absolute top-0 mt-5">
                <h5 class="text-white">{"作成"}</h5>
              </div>
              <div class="position-absolute bottom-0 mb-5">
                <ShutterButton title={""} destination={"#"}/>
              </div>
              </div>
              <video id="camera" width="375" height="667" style="z-index: -1;" class="w-100 h-100 position-absolute"></video>
              <canvas id="picture" width="375" height="667" style="z-index: -1;" class="w-100 h-100 position-absolute"></canvas>
          </div>
        // <div class="container">
        //   <BackButton title={""} destination={"/"}/>
        //   <div  class="d-flex flex-column align-items-center">
        //     <div class="position-absolute top-0 mt-4">
        //       <h3>{"作成"}</h3>
        //     </div>
        //     <div class="position-absolute bottom-0 mb-5">
        //       <ShutterButton title={""} destination={"/view"}/>
        //     </div>
        //   </div>
        // </div>
      </>
    }
}
