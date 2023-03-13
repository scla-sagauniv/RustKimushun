use yew::{function_component, html, Html};

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
