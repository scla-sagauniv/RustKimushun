use yew::{function_component, html, Html, Properties};
use yew_router::{prelude::*, navigator};
use crate::components::button::Button;

#[function_component(Camera)]
pub fn camera() -> Html {
    html! {
      <h1>{"camera"}</h1>
    }
}