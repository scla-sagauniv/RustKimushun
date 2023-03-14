use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)] // Properties, PartialEq を継承した構造体を作成
pub struct ButtonRouter {
    pub title: String,
    pub destination: String,
}

#[function_component(BackButton)]
// pub fn button(props: &button_router) -> Html {
pub fn button(props: &ButtonRouter) -> Html {
    let url = props.destination.clone();
    html! {
      <a href={url}>
        <button type="button" class="btn border-0 px-0" style="color: tomato">
          <i class="fa-regular fa-circle-left fa-2xl"></i>
        </button>
      </a>
    }
}
