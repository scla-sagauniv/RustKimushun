use yew::prelude::*;
use yew_router::{navigator, prelude::*};
mod components;
use components::camera::Camera;
use components::home::Home;
use components::scan::Scan;

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
    #[at("/scan")]
    Scan,
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
        Route::Scan => html! {
            <Scan />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
