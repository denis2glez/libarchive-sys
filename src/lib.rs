#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::{CStr, CString},
        fs::File,
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

            if r != ARCHIVE_OK {
                panic!("archive_read_open_filename fails!");
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

            r = archive_read_free(a) as u32;
            if r != ARCHIVE_OK {
                panic!("archive_read_free fails!");
            }
        }
    }

    // #[test]
    // fn list_in_memory_archive_content() {
    //     unsafe {
    //         let a = archive_read_new();
    //         archive_read_support_compression_gzip(a);
    //         archive_read_support_format_tar(a);
    //     }
    // }

    // #[repr(C)]
    // struct mydata {
    //     name: String,
    //     fd: File,
    // }

    // #[test]
    // fn list_archive_content_custom_function() {
    //     let name = "hello";
    //     let mydata = mydata {
    //         name: name.to_owned(),
    //         fd: File::open(name).unwrap(),
    //     };

    //     unsafe {
    //         let a = archive_read_new();
    //         archive_read_support_compression_all(a);
    //         archive_read_support_format_all(a);

    //         // let c_str = CString::new("data/temp.tar").unwrap();
    //         let mut r = archive_read_open(a, mydata, ptr::null(), myread, myclose) as u32;

    //         if r != ARCHIVE_OK {
    //             panic!("archive_read_open_filename fails!");
    //         }

    //         let mut entry: *mut archive_entry = ptr::null_mut();
    //         while archive_read_next_header(a, &mut entry) == ARCHIVE_OK as i32 {
    //             println!(
    //                 "- {}",
    //                 CStr::from_ptr(archive_entry_pathname(entry))
    //                     .to_str()
    //                     .unwrap()
    //             );
    //             archive_read_data_skip(a);
    //         }

    //         r = archive_read_free(a) as u32;
    //         if r != ARCHIVE_OK {
    //             panic!("archive_read_free fails!");
    //         }
    //     }
    // }
}
