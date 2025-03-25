import { useEffect, useRef } from "react";

const Canvas = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const context = canvas.getContext("2d");
    if (!context) return;

    // Mouse state
    const mouseState = { x: 0, y: 0, isPressed: false };

    // Update mouse state on events
    const updateMousePosition = (event: MouseEvent) => {
      const rect = canvas.getBoundingClientRect();
      mouseState.x = event.clientX - rect.left;
      mouseState.y = event.clientY - rect.top;
    };

    const handleMouseDown = () => (mouseState.isPressed = true);
    const handleMouseUp = () => (mouseState.isPressed = false);

    // Attach event listeners
    window.addEventListener("mousemove", updateMousePosition);
    window.addEventListener("mousedown", handleMouseDown);
    window.addEventListener("mouseup", handleMouseUp);

    // Expose functions to window
    (window as any).mouseX = () => mouseState.x;
    (window as any).mouseY = () => mouseState.y;
    (window as any).mouseIsPressed = () => mouseState.isPressed;

    // Import and initialize WASM module

    //import("../../p5rs-wasm/pkg/p5rs_wasm.js").then(async (module) => {
    //  await module.default(); // Init WASM
    //  module.setup();
    //
    //  function animate() {
    //    module.draw();
    //    requestAnimationFrame(animate);
    //  }
    //
    //  requestAnimationFrame(animate);
    //});

    // Cleanup function
    return () => {
      canvas.removeEventListener("mousemove", updateMousePosition);
      canvas.removeEventListener("mousedown", handleMouseDown);
      canvas.removeEventListener("mouseup", handleMouseUp);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      id="canvas"
      width="1000"
      height="1000"
      style={{
        border: "10px solid black",
        imageRendering: "pixelated", // Apply pixelation
      }}
    ></canvas>
  );
};

export default Canvas;
