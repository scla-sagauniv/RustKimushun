use crate::components::{back_button::BackButton, shutter_button::ShutterButton};
use yew::{function_component, html, Html};

#[function_component(Scan)]
pub fn scan() -> Html {
    html! {
      <>
      <div class="container">
      <BackButton title={""} destination={"/"}/>
      <div  class="d-flex flex-column align-items-center">
        <div class="position-absolute top-0 mt-4">
          <h3>{"QRコードを読み取ってください"}</h3>
        </div>
        <div class="position-absolute bottom-0 mb-5">
          <ShutterButton title={""} destination={"/display"}/>
        </div>
      </div>
    </div>
      </>
    }
}
