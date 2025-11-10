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

#[tauri::command]
fn print_sample_label() -> Result<(), String> {
    // include font from the app assets folder (src/assets/fonts/Amiri-Regular.ttf)
    let font = include_bytes!("../../src/assets/fonts/Amiri-Regular.ttf");
    let epl = zebra_epl2_printer::build_two_product_label_clean(
        font,
        "عصير برتقال صغير    5.00 EGP",
        "622300123456",
        "مياه معدنية صغيرة    3.50 EGP",
        "622300654321",
    );
    // Write the generated EPL to a temp file for debugging (so you can inspect the raw job)
    if let Ok(tmp) = std::env::temp_dir().join("last_epl.bin").into_os_string().into_string() {
        let _ = std::fs::write(&tmp, &epl);
        println!("Wrote EPL debug file: {}", tmp);
    }

    // send to printer named "Zebra LP2824" - adjust if your printer name differs
    zebra_epl2_printer::send_raw_to_printer("Zebra LP2824", &epl).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![greet, print_two_product_label, print_sample_label])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
