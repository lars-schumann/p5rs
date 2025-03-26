import { useState } from "react";
import Editor from "@monaco-editor/react";

const files = {
  "script.js": { name: "script.js", language: "javascript", value: "// JavaScript code here" },
  "style.css": { name: "style.css", language: "css", value: "/* CSS code here */" },
  "index.html": { name: "index.html", language: "html", value: "<!-- HTML code here -->" },
};

function EditorComponent() {
  const [fileName, setFileName] = useState<keyof typeof files>("script.js");

  return (
    <div>
      <button disabled={fileName === "script.js"} onClick={() => setFileName("script.js")}>
        script.js
      </button>
      <button disabled={fileName === "style.css"} onClick={() => setFileName("style.css")}>
        style.css
      </button>
      <button disabled={fileName === "index.html"} onClick={() => setFileName("index.html")}>
        index.html
      </button>

      <Editor
        height="80vh"
        theme="vs-dark"
        path={files[fileName].name}
        defaultLanguage={files[fileName].language}
        defaultValue={files[fileName].value}
      />
    </div>
  );
}

export default EditorComponent;