import { invoke } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
	const handlePrint2 = async () => {
		// For 2-product labels
		await invoke("print_two_product_label", {
			printer: "Zebra LP2824",
			p1_name: "عصير برتقال",
			p1_price: "5.00",
			p1_barcode: "622300123456",
			p2_name: "مياه معدنية",
			p2_price: "3.50",
			p2_barcode: "622300654321",
		});
	};
	const handlePrint4 = async () => {
		// For 4-product labels (2x2 grid)
		await invoke("print_four_product_label", {
			printer: "Zebra LP2824",
			p1_name: "منتج ١",
			p1_price: "5.00",
			p1_barcode: "622300123456",
			p2_name: "منتج ٢",
			p2_price: "3.50",
			p2_barcode: "622300654321",
			p3_name: "منتج ٣",
			p3_price: "7.25",
			p3_barcode: "622300789012",
			p4_name: "منتج ٤",
			p4_price: "2.75",
			p4_barcode: "622300345678",
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
