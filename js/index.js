import("../pkg/index.js").catch(console.error).then((wasm) => {
    wasm.foo(document.body);
});
