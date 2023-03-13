use yew::{function_component, html, Html, Properties};
use yew_router::{prelude::*, navigator};
use crate::components::button::Button;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <>
        // <Button title={"作成"} destination={""}/>
        <Button title={"作成"} destination={"/camera"}/>
      </>
    }
}

