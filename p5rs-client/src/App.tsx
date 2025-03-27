import MonacoEditor from "./MonacoEditor";
import "./App.css";
import Canvas from "./Canvas";
import CodeLoader from "./CodeLoader";

function App() {
  return (
    <div className="app">
      <MonacoEditor />
      <Canvas />
      <CodeLoader />
    </div>
  );
}

export default App;
