(() => {
  const placeholder = { __wbindgen_describe() {} };

  function patchImports(imports) {
    if (imports && typeof imports === "object" && !imports.__wbindgen_placeholder__) {
      imports.__wbindgen_placeholder__ = placeholder;
    }
    return imports;
  }

  const instantiate = WebAssembly.instantiate.bind(WebAssembly);
  WebAssembly.instantiate = (moduleOrBytes, imports) => {
    return instantiate(moduleOrBytes, patchImports(imports));
  };

  if (typeof WebAssembly.instantiateStreaming === "function") {
    const instantiateStreaming = WebAssembly.instantiateStreaming.bind(WebAssembly);
    WebAssembly.instantiateStreaming = (source, imports) => {
      return instantiateStreaming(source, patchImports(imports));
    };
  }

  const Instance = WebAssembly.Instance;
  WebAssembly.Instance = function SparshaPatchedWasmInstance(module, imports) {
    return new Instance(module, patchImports(imports));
  };
  WebAssembly.Instance.prototype = Instance.prototype;
  Object.setPrototypeOf(WebAssembly.Instance, Instance);
})();
