import { invoke } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
  const handlePrint = async () => {
    try {
      await invoke("print_sample_label");
      // eslint-disable-next-line no-alert
      alert("✅ Printed sample label!");
    } catch (e) {
      // eslint-disable-next-line no-alert
      alert("Failed to print: " + String(e));
    }
  };

  return (
    <div style={{ display: "flex", justifyContent: "center", alignItems: "center", height: "100vh", flexDirection: "column" }}>
      <h2>Zebra EPL2 Printer Demo</h2>
  <button type="button" onClick={handlePrint} style={{ fontSize: 16, padding: "12px 20px" }}>🖨️ Print Sample Label</button>
    </div>
  );
}
