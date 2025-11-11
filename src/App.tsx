import { invoke } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
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

	const handlePrint2 = async () => {
		try {
			console.log("Attempting to print 2 products...");
			// For 2-product labels
			await invoke("print_label", {
				printer: "Zebra LP2824",
				brand_name: "اسواق ابوعمر",
				products: [
					{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
					{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
				],
			});
			console.log("2-product print completed successfully!");
		} catch (error) {
			console.error("2-product print failed:", error);
		}
	};

	const handlePrint4 = async () => {
		try {
			console.log("Attempting to print 4 products...");
			// For 4-product labels (2x2 grid)
			await invoke("print_label", {
				printer: "Zebra LP2824",
				brand_name: "اسواق ابوعمر",
				products: [
					{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
					{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
					{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
					{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
				],
			});
			console.log("4-product print completed successfully!");
		} catch (error) {
			console.error("4-product print failed:", error);
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
			<button
				type="button"
				onClick={handleTest}
				style={{ fontSize: 16, padding: "12px 20px", marginBottom: "10px" }}
			>
				Test Tauri Communication
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
		</div>
	);
}
