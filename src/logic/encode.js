import "https://unpkg.com/tesseract.js@v3.0.3/dist/tesseract.min.js";
import init, {
  img_split,
  make_img_bytes_str,
  reconstruction,
} from "./pkg/issue8_wasm.js";
init();

// const response = await fetch("/issue8_wasm_bg.wasm");
// const buf = await response.arrayBuffer();
// const { instance } = await WebAssembly.instantiate(buf);
export async function encode(ctx) {
  const width = 375;
  const height = 667;
  // const cv = document.querySelector("#picture");
  // const context = cv.getContext("2d");
  console.log("OK");
  const img = ctx.getImageData(0, 0, width, height);
  console.log("canvas", img);
  const images /*Vec<Vec<u8>>*/ = JSON.parse(
    img_split(img.data /*&[u8]*/, width /*u32*/, height /*u32*/)
  );
  console.log("split", images);

  let all_img_bytes_str = "";
  let i = 0;
  const tmpCanvas = document.createElement("canvas");
  tmpCanvas.width = width;
  tmpCanvas.height = height / images.length;
  const promises = [];
  for (const image /*Vec<u8>*/ of images) {
    promises.push(
      new Promise((resolve) => {
        const tmpContext = tmpCanvas.getContext("2d");
        const imageData = tmpContext.createImageData(
          tmpCanvas.width,
          tmpCanvas.width
        );
        imageData.data.set(image);
        tmpCanvas.toBlob(async (blob) => {
          const [text, symbols] = await ocr(blob);
          if (!text) {
            resolve();
            return;
          }
          const boxes = [];
          for (const e of symbols) {
            boxes.push([
              e.bbox.x0,
              e.bbox.y0,
              e.bbox.x1 - e.bbox.x0,
              e.bbox.y1 - e.bbox.y0,
            ]);
          }
          const box_str = JSON.stringify({ boxes: boxes });
          console.log("ocr-res", text, box_str);
          const s = make_img_bytes_str(
            image /*&[u8]*/,
            tmpCanvas.width /*u32*/,
            tmpCanvas.height /*u32*/,
            text /*&str*/,
            box_str /*js_sys::Array*/
          );
          console.log(s);
          all_img_bytes_str += s;
          i++;
          resolve();
        });
      })
    );
  }
  await Promise.all(promises);
  console.log("all_str", all_img_bytes_str);
  const orig_img_info = JSON.parse(reconstruction(all_img_bytes_str));
  const cv = document.createElement("canvas");
  cv.width = orig_img_info.width;
  cv.height = orig_img_info.height;
  const tmpContext = cv.getContext("2d");
  const imageData = tmpContext.createImageData(cv.width, cv.width);
  imageData.data.set(orig_img_info.img_bytes);
  tmpCanvas.toBlob(async (blob) => {
    const a = document.createElement("a");
    a.src = URL.createObjectURL(blob);
    a.download = `image${i}.jpg`;
    a.click();
  });
}

const { createWorker } = Tesseract;
async function ocr(img) {
  const worker = createWorker({
    // workerPath: "/node_modules/tesseract.js/dist/worker.min.js",
    // langPath: "/lang-data",
    // corePath: "/node_modules/tesseract.js-core/tesseract-core.wasm.js",
    // logger: (m) => console.log(m),
  });
  const r = async (img) => {
    await worker.load();
    await worker.loadLanguage("eng");
    await worker.initialize("eng");
    await worker.setParameters({
      tessedit_char_whitelist: 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/',
    });
    const {
      data: { text, symbols },
    } = await worker.recognize(
      "https://tesseract.projectnaptha.com/img/eng_bw.png"
    );
    await worker.terminate();
    return [text, symbols];
  };
  return await r(img);
}
