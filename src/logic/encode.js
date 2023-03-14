import "https://unpkg.com/tesseract.js@v3.0.3/dist/tesseract.min.js";

export async function encode(img) {
  const response = await fetch("/issue8_wasm.wasm");
  const buf = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(buf);
  const images /*Vec<DynamicImage>*/ = instance.exports.img_split(
    img /*DynamicImage*/
  );
  let all_img_bytes_str = "";
  for (const image /*DynamicImage*/ of images) {
    const { text, boxs } = ocr(image);
    const s = instance.exports.make_img_bytes_str(
      image /*&DynamicImage*/,
      text /*&String*/,
      boxs /*&[BoxGeometry]*/
    );
    all_img_bytes_str += s;
  }
  instance.exports.reconstruction(all_img_bytes_str);
}

const { createWorker } = Tesseract;
async function ocr(imgPath) {
  const worker = createWorker({
    // workerPath: "/node_modules/tesseract.js/dist/worker.min.js",
    // langPath: "/lang-data",
    // corePath: "/node_modules/tesseract.js-core/tesseract-core.wasm.js",
    logger: (m) => console.log(m),
  });
  console.log(imgPath);
  const recognize = async (imgPath) => {
    await worker.load();
    await worker.loadLanguage("eng");
    await worker.initialize("eng");
    const {
      data: { text, symbols },
    } = await worker.recognize(
      "https://tesseract.projectnaptha.com/img/eng_bw.png"
    );
    await worker.terminate();
    return text, symbols;
  };
  return await recognize(imgPath);
}
