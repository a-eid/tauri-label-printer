// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;

#[derive(Deserialize)]
struct Product {
    name: String,
    price: String,
    barcode: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn print_label(printer: String, brand: Option<String>, products: Vec<Product>) -> Result<(), String> {
    let font = include_bytes!("../../src/assets/fonts/Amiri-Regular.ttf");
    let brand_str = brand.as_deref().unwrap_or("اسواق ابو عمر");
    let data = match products.len() {
        2 => zebra_epl2_printer::build_two_product_label_with_brand(
            font,
            brand_str,
            &products[0].name, &products[0].price, &products[0].barcode,
            &products[1].name, &products[1].price, &products[1].barcode,
        ),
        4 => zebra_epl2_printer::build_four_product_label_with_brand(
            font,
            brand_str,
            &products[0].name, &products[0].price, &products[0].barcode,
            &products[1].name, &products[1].price, &products[1].barcode,
            &products[2].name, &products[2].price, &products[2].barcode,
            &products[3].name, &products[3].price, &products[3].barcode,
        ),
        _ => return Err(format!("Invalid number of products: {}. Expected 2 or 4.", products.len())),
    };
    send_to_printer_cross_os(&printer, &data)
}

#[tauri::command]
fn print_sample_label() -> Result<(), String> {
    // Test with the original 2-product function to see if it has the same issue
    let font = include_bytes!("../../src/assets/fonts/Amiri-Regular.ttf");
    let epl = zebra_epl2_printer::build_two_product_label(
        font,
        "عصير برتقال صغير", "5.00", "622300123456",
        "مياه معدنية صغيرة", "3.50", "622300654321",
    );
    // Write the generated EPL to a temp file for debugging (so you can inspect the raw job)
    if let Ok(tmp) = std::env::temp_dir().join("last_epl.bin").into_os_string().into_string() {
        let _ = std::fs::write(&tmp, &epl);
        println!("Wrote EPL debug file: {}", tmp);
    }

    // send to printer named "Zebra LP2824" (Windows) or write to temp on non-Windows
    send_to_printer_cross_os("Zebra LP2824", &epl)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, print_label, print_sample_label])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}// Cross-OS printer sink: real spool on Windows, temp file elsewhere
fn send_to_printer_cross_os(printer: &str, data: &[u8]) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return zebra_epl2_printer::send_raw_to_printer(printer, data).map_err(|e| e.to_string());
    }
    #[cfg(not(target_os = "windows"))]
    {
        let path = std::env::temp_dir().join("last_epl.bin");
        std::fs::write(&path, data).map_err(|e| e.to_string())
    }
}
