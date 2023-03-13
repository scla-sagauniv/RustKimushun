use yew::{function_component, html, Html, Properties, Callback, MouseEvent};
use yew_router::{prelude::*, navigator};

use yew::prelude::*;

#[function_component(Result)]
pub fn result() -> Html {

    html! {
      <>
        <a href="/view">
          <h1>{"ここに画像"}</h1>
        </a>
      </>
    }
  }
