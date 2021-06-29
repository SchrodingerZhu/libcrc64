use crc64fast::Digest;

#[no_mangle]
pub unsafe extern "C" fn fast_crc64(src: *const u8, size: usize) -> u64 {
    let mut digest = Digest::new();
    digest.write(core::slice::from_raw_parts(src, size));
    digest.sum64()
}


#[no_mangle]
pub unsafe extern "C" fn table_crc64(src: *const u8, size: usize) -> u64 {
    let mut digest = Digest::new_table();
    digest.write(core::slice::from_raw_parts(src, size));
    digest.sum64()
}


#[cfg(test)]
mod test {
    use crate::{fast_crc64, table_crc64};

    #[test]
    fn fast_write() {
        let data = b"hello, world";
        unsafe {
            let res = fast_crc64(data.as_ptr(), data.len());
            assert_eq!(res, 242422964339304333);
        }
    }


    #[test]
    fn table_write() {
        let data = b"hello, world";
        unsafe {
            let res = table_crc64(data.as_ptr(), data.len());
            assert_eq!(res, 242422964339304333);
        }
    }
}