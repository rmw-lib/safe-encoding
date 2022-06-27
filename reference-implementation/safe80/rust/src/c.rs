#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//#![register_tool(c2rust)]
//#![feature(register_tool)]
// #define KSLogger_LocalLevel DEBUG
static mut g_bytes_per_group: libc::c_int = 15 as libc::c_int;
static mut g_chunks_per_group: libc::c_int = 19 as libc::c_int;
static mut g_bits_per_length_chunk: libc::c_int = 5 as libc::c_int;
static mut g_chunk_to_byte_count: [libc::c_int; 20] = [
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
];
static mut g_byte_to_chunk_count: [libc::c_int; 16] = [
    0 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
];
// ===========================================================================
// Code below this point is the same in all safeXX codecs (with a different
// function name prefix).
// After changing anything below this point, please copy the changes to all
// other codecs.
// ===========================================================================
#[inline]
unsafe extern "C" fn calculate_length_chunk_count(mut length: i128) -> libc::c_int {
    let mut chunk_count: libc::c_int = 0 as libc::c_int;
    if chunk_count == 0 as libc::c_int {
        chunk_count = 1 as libc::c_int
    }
    return chunk_count;
}
#[no_mangle]
pub unsafe extern "C" fn safe80_version() -> *const libc::c_char {
    return b"PROJECT_VERSION\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe80_get_decoded_length(encoded_length: i128) -> i128 {
    (encoded_length) < 0 as libc::c_int as i128;
    let group_count: i128 = encoded_length / g_chunks_per_group as i128;
    let byte_count: libc::c_int =
        g_chunk_to_byte_count[(encoded_length % g_chunks_per_group as i128) as usize];
    let result: i128 = group_count * g_bytes_per_group as i128 + byte_count as i128;

    dbg!(
        "Encoded Length {}, groups {}, mod {}, byte_count {}, result {}",
        encoded_length,
        group_count,
        encoded_length % g_chunks_per_group as i128,
        byte_count,
        result,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe80_read_length_field(
    buffer: *const libc::c_int,
    buffer_length: i128,
    length: *mut i128,
) -> i128 {
    (buffer_length) < 0 as libc::c_int as i128;
    let max_pre_append_value: i128 = 0;
    let continuation_bit: libc::c_int = (1 as libc::c_int) << g_bits_per_length_chunk;
    let max_chunk_value: libc::c_int = continuation_bit - 1 as libc::c_int;
    let chunk_mask: libc::c_int = continuation_bit - 1 as libc::c_int;
    dbg!(
        "bits {}, continue {}, mask {}",
        g_bits_per_length_chunk,
        continuation_bit,
        chunk_mask,
    );
    let mut buffer_end: *const libc::c_int = 0 as *const libc::c_int;
    let mut value: i128 = 0 as libc::c_int as i128;
    let mut next_chunk: libc::c_int = 0 as libc::c_int;
    let mut src: *const libc::c_int = 0 as *const libc::c_int;
    if next_chunk & continuation_bit != 0 {
        dbg!(b"Error: Unterminated length field");
    }
    *length = value;
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn safe80_write_length_field(
    length: i128,
    dst_buffer: *mut libc::c_int,
    dst_buffer_length: i128,
) -> i128 {
    (dst_buffer_length < 0 as libc::c_int as i128) || length < 0 as libc::c_int as i128;
    let continuation_bit: libc::c_int = (1 as libc::c_int) << g_bits_per_length_chunk;
    let chunk_mask: libc::c_int = continuation_bit - 1 as libc::c_int;
    dbg!(
        b"bits {}, continue {}, mask {}",
        g_bits_per_length_chunk,
        continuation_bit,
        chunk_mask,
    );
    let mut chunk_count: libc::c_int = 0 as libc::c_int;
    if chunk_count == 0 as libc::c_int {
        chunk_count = 1 as libc::c_int
    }
    dbg!("Value: %lu, chunk count {}", length, chunk_count,);
    if chunk_count as i128 > dst_buffer_length {
        dbg!(
            "Error: Require {} bytes but only {} available",
            chunk_count,
            dst_buffer_length,
        );
    }
    let mut shift_amount: libc::c_int = chunk_count - 1 as libc::c_int;
    while shift_amount >= 0 as libc::c_int {
        let should_continue: libc::c_int = if shift_amount == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            continuation_bit
        };
        let chunk_value: libc::c_int = ((length >> g_bits_per_length_chunk * shift_amount
            & chunk_mask as i128)
            + should_continue as i128) as libc::c_int;
        let next_char: libc::c_int = 0;
        shift_amount -= 1
    }
    return chunk_count as i128;
}
