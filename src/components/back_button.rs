use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)] // Properties, PartialEq を継承した構造体を作成
pub struct button_router {
    pub title: String,
    pub destination: String,
}

#[function_component(BackButton)]
// pub fn button(props: &button_router) -> Html {
pub fn button(props: &button_router) -> Html {
    let url = props.destination.clone();
    html! {
      <a href={url}>
        <button type="button" class="btn btn-outline-light px-0" style="position: absolute; padding: 10px; left: 20px; top: 20px; color: tomato">
          <i class="fa-regular fa-circle-left fa-2xl"></i>
        </button>
      </a>
    }
}
