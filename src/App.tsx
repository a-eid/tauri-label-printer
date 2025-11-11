import { invoke } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
	const handlePrint2 = async () => {
		// For 2-product labels
		await invoke("print_label", {
			printer: "Zebra LP2824",
			title: "أسواق ابوعمر",
			products: [
				{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
				{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
			],
		});
	};

	const handlePrint4 = async () => {
		// For 4-product labels (2x2 grid)
		await invoke("print_label", {
			printer: "Zebra LP2824",
			title: "أسواق ابوعمر",
			products: [
				{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
				{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
				{ name: "عصير برتقال", price: "5.00", barcode: "622300123456" },
				{ name: "مياه معدنية", price: "3.50", barcode: "622300654321" },
			],
		});
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
