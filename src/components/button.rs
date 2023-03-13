use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)] // Properties, PartialEq を継承した構造体を作成
pub struct button_router {
    pub title: String,
    pub destination: String,
}


#[function_component(Button)]
// pub fn button(props: &button_router) -> Html {
pub fn button(props: &button_router) -> Html {  
    let url = props.destination.clone();
    html! {
        <a href={url}>
            <button>{&props.title}</button>
        </a>
    }
}