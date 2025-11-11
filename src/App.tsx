import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css";

export default function App() {
	const [printers, setPrinters] = useState<string[]>([]);
	const [printer, setPrinter] = useState<string>("Zebra LP2824");
	const [brand, setBrand] = useState<string>("اسواق ابوعمر");
	const [lastMsg, setLastMsg] = useState<string>("");

	useEffect(() => {
		// Fetch printers on mount (Windows returns list, other OS returns empty)
		invoke<string[]>("list_printers")
			.then((list) => {
				if (Array.isArray(list) && list.length) {
					setPrinters(list);
					// If our default isn't present, pick the first
					if (!list.includes(printer)) setPrinter(list[0]);
				}
			})
			.catch((e) => console.warn("list_printers failed:", e));
	}, [printer]);
	const handleTest = async () => {
		try {
			console.log("Testing basic Tauri communication...");
			const result = await invoke("greet", { name: "Test" });
			console.log("Greet result:", result);
			alert("Tauri communication works! Result: " + result);
		} catch (error) {
			console.error("Tauri communication failed:", error);
			alert("Error: " + error);
		}
	};

	const handleSimplePrint = async () => {
		try {
			console.log("Testing simple print function...");
			const result = await invoke("print_simple_test");
			console.log("Simple print result:", result);
			alert("Simple print works! Result: " + result);
		} catch (error) {
			console.error("Simple print failed:", error);
			alert("Simple print error: " + error);
		}
	};

	const handlePrint2 = async () => {
		try {
			console.log("Attempting to print 2 products...");
			// For 2-product labels
			await invoke("print_label", {
				printer,
				brand_name: brand,
				products: [
					{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
					{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
				],
			});
			const msg = "2-product print completed successfully!";
			console.log(msg);
			setLastMsg(msg);
		} catch (error) {
			console.error("2-product print failed:", error);
			setLastMsg(`2-product print failed: ${error}`);
		}
	};

	const handlePrint4 = async () => {
		try {
			console.log("Attempting to print 4 products...");
			// For 4-product labels (2x2 grid)
			await invoke("print_label", {
				printer,
				brand_name: brand,
				products: [
					{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
					{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
					{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
					{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
				],
			});
			const msg = "4-product print completed successfully!";
			console.log(msg);
			setLastMsg(msg);
		} catch (error) {
			console.error("4-product print failed:", error);
			setLastMsg(`4-product print failed: ${error}`);
		}
	};

	return (
		<div
			style={{
				display: "flex",
				justifyContent: "center",
				alignItems: "center",
				height: "100vh",
				flexDirection: "column",
			}}
		>
			<h2>Zebra EPL2 Printer Demo</h2>
			<div style={{ marginBottom: 12, display: "flex", gap: 8, alignItems: "center" }}>
				<label htmlFor="printerSelect">Printer:</label>
				<select id="printerSelect" value={printer} onChange={(e) => setPrinter(e.target.value)}>
					{printers.length === 0 ? (
						<option value={printer}>{printer}</option>
					) : (
						printers.map((p) => (
							<option key={p} value={p}>
								{p}
							</option>
						))
					)}
				</select>
			</div>
			<div style={{ marginBottom: 12, display: "flex", gap: 8, alignItems: "center" }}>
				<label htmlFor="brandInput">Brand (leave empty to skip):</label>
				<input id="brandInput" value={brand} onChange={(e) => setBrand(e.target.value)} />
			</div>
			<button
				type="button"
				onClick={handleTest}
				style={{ fontSize: 16, padding: "12px 20px", marginBottom: "10px" }}
			>
				Test Tauri Communication
			</button>
			<button
				type="button"
				onClick={handleSimplePrint}
				style={{ fontSize: 16, padding: "12px 20px", marginBottom: "10px" }}
			>
				Test Simple Print Function
			</button>
			<button
				type="button"
				onClick={handlePrint2}
				style={{ fontSize: 16, padding: "12px 20px" }}
			>
				Print 2 products
			</button>
			<button
				type="button"
				onClick={handlePrint4}
				style={{ fontSize: 16, padding: "12px 20px" }}
			>
				print 4 products
			</button>
			{lastMsg && (
				<div style={{ marginTop: 12, color: lastMsg.includes("failed") ? "#b00020" : "#0a7" }}>
					{lastMsg}
				</div>
			)}
		</div>
	);
}
