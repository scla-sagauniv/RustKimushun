use crate::components::back_button::BackButton;
use yew::{function_component, html, Html};

#[function_component(Camera)]
pub fn camera() -> Html {
    html! {
      <>
        <div class="container">
          <div class="text-center mt-4">
            <h3>{"作成"}</h3>
          </div>
          <BackButton title={""} destination={"/"}/>
        </div>
      </>
    }
}
