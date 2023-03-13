use crate::components::button::Button;
use yew::{function_component, html, Html};

#[function_component(Display)]
pub fn display() -> Html {
    html! {
      <Button title={"ホームへ戻る"} destination={"/"}/>
    }
}
