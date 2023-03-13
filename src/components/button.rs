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
        <a href={url} class="col p-0 mx-auto mt-4 w-50">
            <button type="button" class="btn btn-outline-success w-100 h-100 border border-secondary rounded-3 py-2 h2 fw-bold">
                {&props.title}
            </button>
        </a>
    }
}