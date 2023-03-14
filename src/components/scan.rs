use crate::components::{back_button::BackButton, shutter_button::ShutterButton};
use crate::logic::to_code;
use yew::{function_component, html, Callback, Html, MouseEvent};

#[function_component(Scan)]
pub fn scan() -> Html {
    let origin_img = image::open("cat.png").unwrap();
    let onclick = move |_: MouseEvent| {
        log::info!("log log");
        let display_cords = to_code::to_code(
            origin_img.width(),
            origin_img.height(),
            origin_img.as_bytes(),
        );
        for a_display_code in display_cords.iter() {
            println!(
                "color={:?}::char={}",
                a_display_code.color_code.to_color(),
                a_display_code.char_code.to_char()
            )
        }
    };
    html! {
      <>
      <div class="container">
      <BackButton title={""} destination={"/"}/>
      <div  class="d-flex flex-column align-items-center">
        <div class="position-absolute top-0 mt-4">
          <h3>{"QRコードを読み取ってください"}</h3>
        </div>
        <div class="position-absolute bottom-0 mb-5" onclick={onclick}>
          <ShutterButton title={""} destination={"/display"} />
        </div>
      </div>
    </div>
      </>
    }
}
