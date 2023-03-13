use yew::{function_component, html, Html, Properties};
use yew_router::{prelude::*, navigator};
use crate::components::button::Button;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <>
        <div class="container mt-5 py-5">
          <div class="row d-flex justify-content-center text-center my-5">
            <h1 class="col text-success display-3 fw-bold">{"タイトルタイトル"}</h1>
          </div>
          <div class="row d-flex justify-content-center flex-column mt-5">
            <Button title={"作成"} destination={"/camera"}/>
            <Button title={"読み取り"} destination={"/"}/>
          </div>
        </div>
      </>
    }
}

