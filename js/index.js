import { loader } from "./getResource";

import("../pkg/index.js")
  .then((wasm) => {
    loader.onComplete.add(() => {
      wasm.main_js();
    });
  })
  .catch(console.error);
