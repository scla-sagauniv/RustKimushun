use yew::prelude::*;
use yew_router::{prelude::*, navigator};
mod components;
use components::home::Home;
use components::camera::Camera;


fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/camera")]
    Camera,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home /> 
        },
        Route::Camera => html! {
            <Camera /> 
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

