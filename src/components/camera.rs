use crate::components::back_button::BackButton;
use yew::{function_component, html, Html};

#[function_component(Camera)]
pub fn camera() -> Html {
    html! {
      <BackButton title={""} destination={"/"}/>
    }
}
