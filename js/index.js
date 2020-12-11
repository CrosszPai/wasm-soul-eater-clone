import { loader } from "./getResource";

import("../pkg/index.js")
  .then((wasm) => {
    // laod main after all assets load
    loader.onComplete.add(() => {
      wasm.main_js();
    });
  })
  .catch(console.error);
