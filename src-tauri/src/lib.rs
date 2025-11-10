// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn print_two_product_label(
    printer: String,
    p1_name: String,
    p1_price: String,
    p1_barcode: String,
    p2_name: String,
    p2_price: String,
    p2_barcode: String,
) -> Result<(), String> {
    // include font from the app assets folder (src/assets/fonts/Amiri-Regular.ttf)
    let font = include_bytes!("../../src/assets/fonts/Amiri-Regular.ttf");
    let data = zebra_epl2_printer::build_two_product_label(
        &p1_name,
        &p1_price,
        &p1_barcode,
        &p2_name,
        &p2_price,
        &p2_barcode,
        font,
    );
    zebra_epl2_printer::send_raw_to_printer(&printer, &data).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, print_two_product_label])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
