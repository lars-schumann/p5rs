import { useEffect, useState } from "react";

function ScriptLoaderJsWasm() {
  const [jsModule, setJsModule] = useState<any>(null);
  //const [wasmModule, setWasmModule] = useState<any>(null);

  useEffect(() => {
    // Load WebAssembly module
    //const loadWasmModule = async () => {
    //  try {
    //    const res = await fetch("/p5rs_wasm_bg.wasm");
    //    const wasmArrayBuffer = await res.arrayBuffer();
    //    const wasm = await WebAssembly.instantiate(wasmArrayBuffer);
    //    setWasmModule(wasm.instance.exports);
    //  } catch (err) {
    //    console.error("Failed to load WebAssembly module", err);
    //  }
    //};

    // Load JavaScript module
    const loadJsModule = async () => {
      try {
        const res = await fetch("/scriptA.js");
        const script = await res.text();
        const blob = new Blob([script], { type: "application/javascript" });
        const url = URL.createObjectURL(blob);
        const mod = await import(url);
        setJsModule(mod);
      } catch (err) {
        console.error("Failed to load JavaScript module", err);
      }
    };

    //loadWasmModule();
    loadJsModule();
  }, []);

  const execute = async () => {
    if (jsModule /* && wasmModule */) {
      try {
        // Await the default function if it's async
        await jsModule.default("/p5rs_wasm_bg.wasm");

        // Call setup
        jsModule.setup();

        // Animation loop
        function animate() {
          jsModule.draw();
          requestAnimationFrame(animate);
        }
        animate();
      } catch (error) {
        console.error("Error executing jsModule.default()", error);
      }
    }
  };

  return (
    <div>
      <button onClick={execute} disabled={!jsModule /*|| !wasmModule*/}>
        Execute JS with WASM
      </button>
    </div>
  );
}

export default ScriptLoaderJsWasm;
