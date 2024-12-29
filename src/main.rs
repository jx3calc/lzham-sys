use lzham_sys::*;

#[test]
fn hello() {
    let input_data = b"Hello, LZHAM!";
    let input_len = input_data.len() as lzham_z_ulong;

    // Allocate memory for compressed data
    let mut compressed_data = vec![0u8; input_len as usize * 2];
    let mut compressed_len = compressed_data.len() as lzham_z_ulong;

    // Compress the data
    let compress_result = unsafe {
        lzham_z_compress(
            compressed_data.as_mut_ptr(),
            &mut compressed_len,
            input_data.as_ptr(),
            input_len,
        )
    };

    if compress_result != 0 {
        println!("Compression failed with code: {}", compress_result);
        return;
    }

    // Allocate memory for decompressed data
    let mut decompressed_data = vec![0u8; input_len as usize];
    let mut decompressed_len = decompressed_data.len() as lzham_z_ulong;

    // Decompress the data
    let uncompress_result = unsafe {
        lzham_z_uncompress(
            decompressed_data.as_mut_ptr(),
            &mut decompressed_len,
            compressed_data.as_ptr(),
            compressed_len,
        )
    };

    if uncompress_result != 0 {
        println!("Decompression failed with code: {}", uncompress_result);
        return;
    }

    assert_eq!(&decompressed_data[..decompressed_len as usize], input_data);
}

fn main() {
    hello();
}
