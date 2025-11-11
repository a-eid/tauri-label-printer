// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;

#[derive(Deserialize)]
struct Product {
    name: String,
    price: String,
    barcode: String,
}

#[tauri::command]
fn print_label(printer: String, title: Option<String>, products: Vec<Product>) -> Result<(), String> {
    let font = include_bytes!("../../src/assets/fonts/Amiri-Regular.ttf");
    let title_str = title.as_deref().unwrap_or("اسواق ابو عمر");
    let data = match products.len() {
        2 => zebra_epl2_printer::build_two_product_label_with_brand(
            font,
            title_str,
            &products[0].name, &products[0].price, &products[0].barcode,
            &products[1].name, &products[1].price, &products[1].barcode,
        ),
        4 => zebra_epl2_printer::build_four_product_label_with_brand(
            font,
            title_str,
            &products[0].name, &products[0].price, &products[0].barcode,
            &products[1].name, &products[1].price, &products[1].barcode,
            &products[2].name, &products[2].price, &products[2].barcode,
            &products[3].name, &products[3].price, &products[3].barcode,
        ),
        _ => return Err(format!("Invalid number of products: {}. Expected 2 or 4.", products.len())),
    };
    send_to_printer_cross_os(&printer, &data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![print_label])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Cross-OS printer sink: real spool on Windows, temp file elsewhere
fn send_to_printer_cross_os(printer: &str, data: &[u8]) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return zebra_epl2_printer::send_raw_to_printer(printer, data).map_err(|e| e.to_string());
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _printer = printer; // Unused on non-Windows
        let path = std::env::temp_dir().join("last_epl.bin");
        std::fs::write(&path, data).map_err(|e| e.to_string())
    }
}
