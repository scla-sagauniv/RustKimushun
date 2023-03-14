import "https://unpkg.com/tesseract.js@v3.0.3/dist/tesseract.min.js";

const { createWorker } = Tesseract;
export async function ocr(imgPath) {
  console.log("start");
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
