import { useState } from "react";

function CodeLoader() {
  const [jsModule, setJsModule] = useState<any>(null);
  const [wasmModule, setWasmModule] = useState<any>(null);

  const loadJsModule = async () => {
    try {
      const res = await fetch("/code/example/js");
      const script = await res.text();
      const blob = new Blob([script], { type: "application/javascript" });
      const url = URL.createObjectURL(blob);
      const mod = await import(url);
      setJsModule(mod);
    } catch (err) {
      console.error("Failed to load JavaScript module", err);
    }
  };

  const loadWasmModule = async () => {
    if (!jsModule) {
      console.error("JavaScript module must be loaded first");
      return;
    }
    try {
      await jsModule.default("/code/example/wasm");
      jsModule.setup();
      setWasmModule(jsModule);
    } catch (error) {
      console.error("Error loading WebAssembly module", error);
    }
  };

  const execute = async () => {
    if (wasmModule) {
      try {
        function animate() {
          wasmModule.draw();
          requestAnimationFrame(animate);
        }
        animate();
      } catch (error) {
        console.error("Error executing wasmModule.draw()", error);
      }
    }
  };

  return (
    <div>
      <button onClick={loadJsModule} disabled={!!jsModule}>
        Load JavaScript Module
      </button>
      <button onClick={loadWasmModule} disabled={!jsModule || !!wasmModule}>
        Load WebAssembly Module
      </button>
      <button onClick={execute} disabled={!wasmModule}>
        Run WebAssembly Module
      </button>
    </div>
  );
}

export default CodeLoader;
