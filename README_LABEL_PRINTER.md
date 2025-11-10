# zebra-epl2-printer (demo in this workspace)

This workspace contains a Tauri app demo and a small Rust library under `src-tauri/src/zebra_epl2_printer` that can render Arabic text, produce EAN-13 barcodes and build EPL2 label bytes.

Quick start (dev):

1. Add `Amiri-Regular.ttf` to `src-tauri/assets/Amiri-Regular.ttf` (download from the Amiri project).
2. From project root run:

```bash
# install frontend deps (pnpm/yarn/npm depending on your setup) and then:
pnpm install
pnpm tauri dev
```

Notes:
- On Windows the Rust function `send_raw_to_printer` attempts to use the Windows spooler APIs. On non-Windows platforms it writes the EPL bytes to a temp file for inspection.
- The library is pure Rust and is implemented inside `src-tauri/src/zebra_epl2_printer` for this demo. It can be extracted as a separate crate if needed.

Files of interest:
- `src-tauri/src/zebra_epl2_printer/graphics.rs` — Arabic shaping + render to bitmap.
- `src-tauri/src/zebra_epl2_printer/barcode.rs` — EAN-13 checksum and EPL barcode command.
- `src-tauri/src/zebra_epl2_printer/epl.rs` — EPL helpers and header.
- `src-tauri/src/zebra_epl2_printer/builder.rs` — high-level label builder.
- `src-tauri/src/zebra_epl2_printer/printer.rs` — send to Windows spooler or write to temp file.
- `src/App.tsx` — simple UI to enter two products and print.

Example usage (Rust):

```rust
// inside src-tauri/src/lib.rs or any Rust binary that depends on the library
let font = std::fs::read("src-tauri/assets/Amiri-Regular.ttf").unwrap();
let epl = zebra_epl2_printer::build_two_product_label(
    "عصير برتقال صغير", "5.00", "622300123456",
    "مياه معدنية صغيرة", "3.50", "622300654321",
    &font,
);
// send to printer name (Windows) or write to temp file on other OS
zebra_epl2_printer::send_raw_to_printer("Zebra LP2824", &epl).unwrap();
```

If you want me to extract the library as a separate crate (workspace member) and wire it into `src-tauri` as a dependency, I can do that next.
