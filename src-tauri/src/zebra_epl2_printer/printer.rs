use std::io::{self, Write};

#[cfg(windows)]
mod win_printer {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::winspool::{OpenPrinterW, StartDocPrinterW, StartPagePrinter, WritePrinter, EndPagePrinter, EndDocPrinter, ClosePrinter, DOC_INFO_1W};

    pub fn send(printer_name: &str, data: &[u8]) -> std::io::Result<()> {
        unsafe {
            let wide: Vec<u16> = OsStr::new(printer_name).encode_wide().chain(once(0)).collect();
            let mut ph = null_mut();
            if OpenPrinterW(wide.as_ptr() as *mut _, &mut ph, null_mut()) == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "OpenPrinter failed"));
            }

            let doc_name = "zebra_epl2_print".encode_utf16().chain(once(0)).collect::<Vec<u16>>();
            let mut di = DOC_INFO_1W { pDocName: doc_name.as_ptr() as *mut _, pOutputFile: std::ptr::null_mut(), pDataType: "RAW\0".encode_utf16().collect::<Vec<u16>>().as_ptr() as *mut _ };

            if StartDocPrinterW(ph, 1, &mut di as *mut _ as *mut _) == 0 {
                ClosePrinter(ph);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "StartDocPrinter failed"));
            }

            if StartPagePrinter(ph) == 0 {
                EndDocPrinter(ph);
                ClosePrinter(ph);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "StartPagePrinter failed"));
            }

            let mut written: DWORD = 0;
            let res = WritePrinter(ph, data.as_ptr() as *mut _, data.len() as DWORD, &mut written);

            EndPagePrinter(ph);
            EndDocPrinter(ph);
            ClosePrinter(ph);

            if res == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "WritePrinter failed"));
            }

            Ok(())
        }
    }
}

/// Send EPL bytes to named printer. On Windows uses the spooler; on other platforms writes to a file in temp.
pub fn send_raw_to_printer(printer_name: &str, data: &[u8]) -> io::Result<()> {
    #[cfg(windows)]
    {
        return win_printer::send(printer_name, data);
    }

    #[cfg(not(windows))]
    {
        // Fallback for non-windows: write to temp file for debugging/testing.
        let mut path = std::env::temp_dir();
        let file_name = format!("{}_epl_output.bin", sanitize_printer_name(printer_name));
        path.push(file_name);
        let mut f = std::fs::File::create(&path)?;
        f.write_all(data)?;
        println!("Wrote EPL to {:?}", path);
        Ok(())
    }
}

fn sanitize_printer_name(s: &str) -> String {
    s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect()
}
