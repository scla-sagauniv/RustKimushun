use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)] // Properties, PartialEq を継承した構造体を作成
pub struct button_router {
    pub title: String,
    pub destination: String,
}

#[function_component(ShutterButton)]
// pub fn button(props: &button_router) -> Html {
pub fn button(props: &button_router) -> Html {
    let url = props.destination.clone();
    html! {
        <a href={url}>
            <button type="button" class="btn btn-outline-light" style="color: tomato;">
              <span class="fa-stack fa-2x">
                <i class="fa-solid fa-camera fa-stack-1x"></i>
                <i class="fa-regular fa-circle fa-stack-2x"></i>
              </span>
            </button>
        </a>
    }
}
