import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [printer, setPrinter] = useState("Zebra LP2824");
  const [p1Name, setP1Name] = useState("");
  const [p1Price, setP1Price] = useState("");
  const [p1Barcode, setP1Barcode] = useState("");
  const [p2Name, setP2Name] = useState("");
  const [p2Price, setP2Price] = useState("");
  const [p2Barcode, setP2Barcode] = useState("");
  const [msg, setMsg] = useState("");

  async function printLabel() {
    setMsg("Printing...");
    try {
      const res: string = await invoke("print_two_product_label", {
        printer,
        p1_name: p1Name,
        p1_price: p1Price,
        p1_barcode: p1Barcode,
        p2_name: p2Name,
        p2_price: p2Price,
        p2_barcode: p2Barcode,
      });
      setMsg(`✅ ${res}`);
    } catch (e: any) {
      setMsg(`❌ Failed: ${e?.toString?.() ?? String(e)}`);
    }
  }

  return (
    <main className="container">
      <h1>Label Printer</h1>
      <div className="form-row">
        <label>Printer name</label>
        <input value={printer} onChange={(e) => setPrinter(e.currentTarget.value)} />

        <h3>Product 1</h3>
        <input placeholder="Name (Arabic)" value={p1Name} onChange={(e) => setP1Name(e.currentTarget.value)} />
        <input placeholder="Price" value={p1Price} onChange={(e) => setP1Price(e.currentTarget.value)} />
        <input placeholder="EAN-13" value={p1Barcode} onChange={(e) => setP1Barcode(e.currentTarget.value)} />

        <h3>Product 2</h3>
        <input placeholder="Name (Arabic)" value={p2Name} onChange={(e) => setP2Name(e.currentTarget.value)} />
        <input placeholder="Price" value={p2Price} onChange={(e) => setP2Price(e.currentTarget.value)} />
        <input placeholder="EAN-13" value={p2Barcode} onChange={(e) => setP2Barcode(e.currentTarget.value)} />

        <button onClick={printLabel}>Print Label</button>

        <div className="toast">{msg}</div>
      </div>
    </main>
  );
}

export default App;
