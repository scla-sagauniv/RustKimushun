use yew::prelude::*;
use yew_router::prelude::*;
mod components;
pub mod logic;
use components::camera::Camera;
use components::display::Display;
use components::home::Home;
use components::result::Result;
use components::scan::Scan;
use components::view::View;

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
    #[at("/display")]
    Display,
    #[at("/view")]
    View,
    #[at("/result")]
    Result,
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
        Route::Display => html! {
            <Display />
        },
        Route::View => html! {
            <View />
        },
        Route::Result => html! {
            <Result />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
