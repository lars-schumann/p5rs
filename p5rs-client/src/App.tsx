import EditorComponent from "./MonacoEditor";
import "./App.css";
import Canvas from "./Canvas";
import ScriptLoaderJsWasm from "./ScriptLoaderJsWasm";

function App() {
  return (
    <div className="app">
      <EditorComponent />
      <Canvas />
      <ScriptLoaderJsWasm />
    </div>
  );
}

export default App;
