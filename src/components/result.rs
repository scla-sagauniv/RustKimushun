use yew::{function_component, html, Html};

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[function_component(Result)]
pub fn result() -> Html {
    let sample1 = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let sample2 = Color { r: 0, g: 200, b: 0 };
    let sample3 = Color { r: 0, g: 0, b: 0 };
    let colors: Vec<Color> = vec![sample1, sample2, sample3];
    let mut styles: Vec<String> = vec![];
    for color in colors {
        styles.push(format!("color: rgb({}, {}, {})", color.r, color.g, color.b));
    }
    html! {
        <>
        <a href="/view">
          <h1>{"ここに画像"}</h1>
        </a>
        {
          for styles.iter().map(
            |style| html!{<b class="fs-1" style={String::from(style) + "; " + "text-shadow: 1px 1px 8px gray;"}>{"Cc"}</b>}
          )
        }
      </>
    }
}
