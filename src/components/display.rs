// use crate::components::button::Button;
// use yew::{function_component, html, Html};

// #[function_component(Display)]
// pub fn display() -> Html {
//     html! {

//       <Button title={"ホームへ戻る"} destination={"/"}/>
//     }
// }

use gloo_file::{callbacks::FileReader, File};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use crate::components::{button::Button};

#[function_component(Display)]
pub fn display() -> Html {
    html! {
        <>        
          <div class="d-flex justify-content-center align-items-center" style="width: 100vw; height: 100vh;">
            <div class="d-flex flex-column align-items-center style=max-height: 100vh;">
              <div class="d-flex justify-content-center text-center my-2">
                <h1 class="text-success px-3 display-3 fw-bold">{"これはQRコードを読み取った後の画面"}<br/>{"下のファイル選択ボタンは仮の"}</h1>
              </div>
              <DisplayDataComponent/>
            </div>
          </div>
        </>
    }
}

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
}
pub struct DisplayDataComponent {
    files: Vec<String>,
    readers: HashMap<String, FileReader>,
}
impl Component for DisplayDataComponent {
    type Message = Msg;
    type Properties = ();
    // データ型作成
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files: Vec::new(),
            readers: HashMap::default(),
        }
    }
    // ファイル更新処理
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();
                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::LoadedBytes(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::LoadedBytes(file_name, data) => {
                log::info!("Processing: {}", file_name);
                let image_data = base64::encode(data);

                // 古いファイルをポップする処理
                let x = self.files.len();
                if x == 0 {
                  self.files.push(image_data);
                }else {
                  self.files.push(image_data);
                  self.files.remove(0);
                }
                true
            }
        }
    }
    // ファイル表示処理
    fn view(&self, ctx: &Context<Self>) -> Html {
      let x = self.files.len();
        let on_change = ctx.link().callback(move |e: Event| {
            let mut selected_files = Vec::new();
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                selected_files.extend(files);
            }
            Msg::Files(selected_files)
        });
        html! {
            <div class="row d-flex justify-content-center flex-column mt-5 px-3">
              <div class="d-flex justify-content-center">
                { for self.files.iter().map(|f| Self::display_file(f))}
              </div>

              <label class="btn btn-outline-success border border-secondary rounded-3 py-2 h2 fw-bold mx-auto mt-4">
                {"ファイルを選択"}
                <input type="file" accept="image/png, image/gif" onchange={on_change} multiple=false style="display: none;"/>
              </label>
              if x == 0 {
              } else {
                <Button title={"ホームへ戻る"} destination={"/"}/>
              }
            </div>
        }
    }
}
impl DisplayDataComponent {
    fn display_file(data: &str) -> Html {
        let img = format!("data:image/png;base64,{}", data.to_string());
        html! {
          <img src={img} style="max-height: 30vh; max-width: 70vw; border: 5px solid #198754; border-radius: 7px;"/>
        }
    }
}
