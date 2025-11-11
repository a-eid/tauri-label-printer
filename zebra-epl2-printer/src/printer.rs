use std::error::Error;

/// Send raw bytes to the named printer. On non-Windows this function returns an error.
pub fn send_raw_to_printer(printer_name: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        use winapi::um::winspool::*;
        use winapi::shared::minwindef::*;
        use winapi::shared::ntdef::LPWSTR;
        use std::ptr::null_mut;
        use winapi::um::errhandlingapi::GetLastError;
        use winapi::um::winbase::FormatMessageW;
        use winapi::um::winbase::{FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
        use std::os::windows::ffi::OsStringExt;
        use std::ffi::OsString;

        // convert printer name and helper strings to wide
        let wide_name: Vec<u16> = OsStr::new(printer_name).encode_wide().chain(once(0)).collect();
        let wide_doc: Vec<u16> = OsStr::new("EPL Job").encode_wide().chain(once(0)).collect();
        let wide_raw: Vec<u16> = OsStr::new("RAW").encode_wide().chain(once(0)).collect();

        unsafe {
            let mut handle: *mut winapi::ctypes::c_void = null_mut();
            if OpenPrinterW(wide_name.as_ptr() as LPWSTR, &mut handle as *mut _ as *mut _, null_mut()) == 0 {
                let msg = last_error_string();
                return Err(format!("OpenPrinterW failed for '{}': {}", printer_name, msg).into());
            }

            let doc_info = DOC_INFO_1W {
                pDocName: wide_doc.as_ptr() as LPWSTR,
                pOutputFile: null_mut(),
                pDatatype: wide_raw.as_ptr() as LPWSTR, // RAW data type
            };

            let job = StartDocPrinterW(handle as *mut _, 1, &doc_info as *const _ as *mut _);
            if job == 0 {
                let msg = last_error_string();
                ClosePrinter(handle as *mut _);
                return Err(format!("StartDocPrinterW failed: {}", msg).into());
            }

            if StartPagePrinter(handle as *mut _) == 0 {
                let msg = last_error_string();
                EndDocPrinter(handle as *mut _);
                ClosePrinter(handle as *mut _);
                return Err(format!("StartPagePrinter failed: {}", msg).into());
            }

            let mut written: DWORD = 0;
            let ok = WritePrinter(
                handle as *mut _,
                data.as_ptr() as *mut _,
                data.len() as DWORD,
                &mut written as *mut DWORD,
            );

            EndPagePrinter(handle as *mut _);
            EndDocPrinter(handle as *mut _);
            ClosePrinter(handle as *mut _);

            if ok == 0 {
                let msg = last_error_string();
                return Err(format!("WritePrinter failed (wrote {} of {} bytes): {}", written, data.len(), msg).into());
            }
            if written as usize != data.len() {
                return Err(format!("Partial write: wrote {} of {} bytes", written, data.len()).into());
            }
            Ok(())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(Box::<dyn Error>::from("send_raw_to_printer is only supported on Windows (uses Win32 spooler)"))
    }
}

/// List installed printers (Windows only). On other platforms returns an empty list.
pub fn list_printers() -> Result<Vec<String>, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use std::ptr::null_mut;
        use winapi::shared::minwindef::DWORD;
        use winapi::um::winspool::*;

        unsafe {
            let mut bytes_needed: DWORD = 0;
            let mut printers_returned: DWORD = 0;

            // First call to get required buffer size
            EnumPrintersW(
                PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS,
                null_mut(),
                4,
                null_mut(),
                0,
                &mut bytes_needed,
                &mut printers_returned,
            );

            if bytes_needed == 0 {
                return Ok(Vec::new());
            }

            let mut buffer: Vec<u8> = vec![0u8; bytes_needed as usize];
            let ok = EnumPrintersW(
                PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS,
                null_mut(),
                4,
                buffer.as_mut_ptr(),
                bytes_needed,
                &mut bytes_needed,
                &mut printers_returned,
            );
            if ok == 0 {
                return Err("EnumPrintersW failed".into());
            }

            let mut result = Vec::new();
            let printer_info_ptr = buffer.as_ptr() as *const PRINTER_INFO_4W;
            for i in 0..printers_returned as isize {
                let info = printer_info_ptr.offset(i).as_ref().unwrap();
                if !info.pPrinterName.is_null() {
                    // Read wide string until null terminator
                    let mut len = 0;
                    loop {
                        let ch = *info.pPrinterName.offset(len);
                        if ch == 0 { break; }
                        len += 1;
                    }
                    let slice = std::slice::from_raw_parts(info.pPrinterName, len as usize);
                    let os = OsString::from_wide(slice);
                    if let Some(s) = os.to_str() {
                        result.push(s.to_string());
                    }
                }
            }
            Ok(result)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(Vec::new())
    }
}

#[cfg(target_os = "windows")]
fn last_error_string() -> String {
    unsafe {
        let err = GetLastError();
        if err == 0 { return "Unknown error".to_string(); }
        let mut buf: [u16; 1024] = [0; 1024];
        let len = FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            std::ptr::null(),
            err,
            0,
            buf.as_mut_ptr(),
            buf.len() as u32,
            std::ptr::null_mut(),
        );
        if len == 0 { return format!("OS Error {}", err); }
        let s = OsString::from_wide(&buf[..len as usize]).to_string_lossy().to_string();
        format!("{} (code {})", s.trim(), err)
    }
}
