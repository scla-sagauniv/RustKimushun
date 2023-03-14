use gloo_file::{callbacks::FileReader, File};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(Camera)]
pub fn camera() -> Html {
    html! {
        <>
        <div class="d-flex justify-content-center align-items-center" style="width: 100vw; height: 100vh;">
          <div class="d-flex flex-column align-items-center style=max-height: 100vh;">
            <div class="d-flex justify-content-center text-center my-2">
              <h1 class="text-success display-3 fw-bold">{"タイトルタイトル"}</h1>
            </div>
            <FileDataComponent/>
          </div>
        </div>
        </>
    }
}

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
}
pub struct FileDataComponent {
    files: Vec<String>,
    readers: HashMap<String, FileReader>,
}
impl Component for FileDataComponent {
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
                { for self.files.iter().map(|f| Self::view_file(f))}
              </div>

              <label class="btn btn-outline-success border border-secondary rounded-3 py-2 h2 fw-bold mx-auto mt-4">
                {"ファイルを選択"}
                <input type="file" accept="image/png, image/gif" onchange={on_change} multiple=false style="display: none;"/>
              </label>
              if x == 0 {
              } else {
                <button class="btn btn-outline-success border border-secondary rounded-3 py-2 h2 fw-bold mx-auto mt-4">
                  {"決定"}
                </button>
              }
            </div>
        }
    }
}
impl FileDataComponent {
    fn view_file(data: &str) -> Html {
        let img = format!("data:image/png;base64,{}", data.to_string());
        html! {
          <img src={img} style="max-height: 30vh; max-width: 70vw;"/>
        }
    }
}




// use crate::components::button::Button;
// use yew::{function_component, html, Html, use_effect_with_deps};

// #[function_component(Camera)]
// pub fn camera() -> Html {
//     html! {
//       <>
//         <div class="container mt-5 py-5">
//           <div class="row d-flex justify-content-center text-center my-5">
//             <h1 class="col text-success display-3 fw-bold">{"タイトルタイトル"}</h1>
//           </div>
//           <div class="row d-flex justify-content-center flex-column mt-5">
//             <Button title={"ファイルを選択"} destination={"/camera"}/>
//           </div>
//         </div>
//       </>
//     }
// }
