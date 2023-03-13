use yew::{function_component, html, Html, Properties, Callback, MouseEvent};
use yew_router::{prelude::*, navigator};
use crate::components::button::Button;

use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {

    html! {
      <>
          <div class="container mt-5 py-5">
            <div class="row d-flex justify-content-center text-center my-5">
              <h1 class="col text-success display-3 fw-bold">{"QRコードが"}<br/>{"完成しました！"}</h1>
            </div>
            <div class="row d-flex justify-content-center flex-column mt-5">
              <Button title={"表示"} destination={"/result"}/>
              <Button title={"完了"} destination={"/"}/>
            </div>
          </div>
      </>
    }
  }
