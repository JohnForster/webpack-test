import("../pkg/index.js")
  .catch(console.error)
  .then((wasm) => {
    alert(wasm.foo("test"));
  });
