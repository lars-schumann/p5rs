import { useEffect, useState } from "react";

function ScriptLoaderJsWasm() {
  const [jsModule, setJsModule] = useState<any>(null);

  useEffect(() => {
    const loadJsModule = async () => {
      try {
        const res = await fetch("/scripts/wasm_loader.js");
        const script = await res.text();
        const blob = new Blob([script], { type: "application/javascript" });
        const url = URL.createObjectURL(blob);
        const mod = await import(url);
        setJsModule(mod);
      } catch (err) {
        console.error("Failed to load JavaScript module", err);
      }
    };

    loadJsModule();
  }, []);

  const execute = async () => {
    if (jsModule) {
      try {
        await jsModule.default("/scripts/unnamed.wasm");
        jsModule.setup();

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
      <button onClick={execute} disabled={!jsModule}>
        Load and run wasm module
      </button>
    </div>
  );
}

export default ScriptLoaderJsWasm;
