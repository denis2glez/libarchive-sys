#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::{CStr, CString},
        ptr,
    };

    #[test]
    fn list_archive_content() {
        unsafe {
            let a = archive_read_new();
            archive_read_support_filter_all(a);
            archive_read_support_format_all(a);

            let c_str = CString::new("data/temp.tar").unwrap();
            let mut r = archive_read_open_filename(a, c_str.as_ptr() as *const i8, 10240) as u32;
            // Note 1

            if r != ARCHIVE_OK {
                panic!("Fails on archive_read_open_filename!");
            }

            let mut entry: *mut archive_entry = ptr::null_mut();
            while archive_read_next_header(a, &mut entry) == ARCHIVE_OK as i32 {
                println!(
                    "- {}",
                    CStr::from_ptr(archive_entry_pathname(entry))
                        .to_str()
                        .unwrap()
                );
                archive_read_data_skip(a);
            }

            r = archive_read_free(a) as u32; // Note 3
            if r != ARCHIVE_OK {
                panic!("Fails on archive_read_free!");
            }
        }
    }
}
