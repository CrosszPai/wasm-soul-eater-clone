import { loader } from "./getResource";

import("../pkg/index.js")
  .then((wasm) => {
    // laod main after all assets load
    loader.onComplete.add(() => {
      wasm.main_js();
    });
    refreshLoop();
  })
  .catch(console.error);

const times = [];
var fps = 0;

function refreshLoop() {
  window.requestAnimationFrame(() => {
    const now = performance.now();
    while (times.length > 0 && times[0] <= now - 1000) {
      times.shift();
    }
    times.push(now);
    fps = times.length;
    document.getElementById("fps").innerHTML = fps.toFixed(2)
    refreshLoop();
  });
}
