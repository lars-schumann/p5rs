import { useEffect, useState } from "react";

function DynamicScriptLoader() {
  const [module, setModule] = useState(null);

  useEffect(() => {
    fetch("/dynamic-script.js")
      .then((res) => res.text())
      .then((script) => {
        const blob = new Blob([script], { type: "application/javascript" });
        const url = URL.createObjectURL(blob);
        import(url).then((mod) => {
          setModule(mod);
        });
      })
      .catch((err) => console.error("Failed to load script", err));
  }, []);

  return (
    <div>
      <button onClick={() => module && (module as any).dynamicFunction()}>
        Execute Function
      </button>
    </div>
  );
}

export default DynamicScriptLoader;
