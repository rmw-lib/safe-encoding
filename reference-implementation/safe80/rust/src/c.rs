#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type safe80_status = i32;
pub const SAFE80_ERROR_NOT_ENOUGH_ROOM: safe80_status = -6;
pub const SAFE80_ERROR_INVALID_LENGTH: safe80_status = -5;
pub const SAFE80_ERROR_TRUNCATED_DATA: safe80_status = -4;
pub const SAFE80_ERROR_UNTERMINATED_LENGTH_FIELD: safe80_status = -3;
pub const SAFE80_ERROR_INVALID_SOURCE_DATA: safe80_status = -2;
pub const SAFE80_STATUS_PARTIALLY_COMPLETE: safe80_status = -1;
pub const SAFE80_STATUS_OK: safe80_status = 0;
pub type safe80_stream_state = libc::c_uint;
pub const SAFE80_DST_IS_AT_END_OF_STREAM: safe80_stream_state = 4;
pub const SAFE80_SRC_IS_AT_END_OF_STREAM: safe80_stream_state = 2;
pub const SAFE80_EXPECT_DST_STREAM_TO_END: safe80_stream_state = 1;
pub const SAFE80_STREAM_STATE_NONE: safe80_stream_state = 0;
static mut g_bytes_per_group: i32 = 15 as i32;
static mut g_chunks_per_group: i32 = 19 as i32;
static mut g_bits_per_byte: i32 = 8 as i32;
static mut g_factor_per_chunk: i32 = 80 as i32;
static mut g_bits_per_length_chunk: i32 = 5 as i32;
static mut g_encode_char_to_chunk: [u8; 256] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xfe, 0xff, 0xff, 0xfe, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xfe, 0, 0xff, 0xff, 0x1, 0xff, 0xff, 0xff, 0x2, 0x3, 0xff, 0x4, 0x5, 0x6, 0xff, 0xff, 0x7,
    0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0xff, 0x11, 0xff, 0x12, 0xff, 0xff, 0x13, 0x14,
    0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24,
    0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0xff, 0x2f, 0x30, 0x31, 0x32, 0x33,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43,
    0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0xff, 0x4e, 0x4f, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];
static mut g_chunk_to_encode_char: [u8; 80] = [
    b'!', b'$', b'(', b')', b'+', b',', b'-', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
    b'9', b';', b'=', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
    b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[', b']',
    b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm',
    b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'{', b'}', b'~',
];
static mut g_chunk_to_byte_count: [i32; 20] = [
    0 as i32, 0 as i32, 1 as i32, 2 as i32, 3 as i32, 3 as i32, 4 as i32, 5 as i32, 6 as i32,
    7 as i32, 7 as i32, 8 as i32, 9 as i32, 10 as i32, 11 as i32, 11 as i32, 12 as i32, 13 as i32,
    14 as i32, 15 as i32,
];
static mut g_byte_to_chunk_count: [i32; 16] = [
    0 as i32, 2 as i32, 3 as i32, 4 as i32, 6 as i32, 7 as i32, 8 as i32, 9 as i32, 11 as i32,
    12 as i32, 13 as i32, 14 as i32, 16 as i32, 17 as i32, 18 as i32, 19 as i32,
];
#[inline]
unsafe fn accumulate_byte(accumulator: i128, byte_value: u8) -> i128 {
    let new_accumulator: i128 = accumulator << g_bits_per_byte | byte_value as i128;
    return new_accumulator;
}
#[inline]
unsafe fn accumulate_chunk(accumulator: i128, next_chunk: u8) -> i128 {
    let new_accumulator: i128 = accumulator * g_factor_per_chunk as i128 + next_chunk as i128;
    return new_accumulator;
}
#[inline]
unsafe fn extract_byte_from_accumulator(accumulator: i128, byte_index_lo_first: i32) -> i32 {
    let byte_mask: i32 = ((1 as i32) << g_bits_per_byte) - 1 as i32;
    let shift_amount: i32 = byte_index_lo_first * g_bits_per_byte;
    let extracted_byte: i32 = (accumulator >> shift_amount) as i32 & byte_mask;
    return extracted_byte;
}
#[inline]
unsafe fn extract_chunk_from_accumulator(accumulator: i128, chunk_index_lo_first: i32) -> i32 {
    static mut is_initialized: bool = 0 as i32 != 0;
    static mut divide_amounts: [i128; 19] = [0; 19];
    if !is_initialized {
        divide_amounts[0 as i32 as usize] = 0 as i32 as i128;
        let mut initializer: i128 = g_factor_per_chunk as i128;
        let mut i: i32 = 1 as i32;
        while i < g_chunks_per_group {
            divide_amounts[i as usize] = initializer;
            initializer *= g_factor_per_chunk as i128;
            i += 1;
        }
        is_initialized = 1 as i32 != 0;
    }
    let chunk_modulo: i32 = g_factor_per_chunk;
    if chunk_index_lo_first == 0 as i32 {
        let extracted_chunk: i32 = (accumulator % chunk_modulo as i128) as i32;
        return extracted_chunk;
    }
    let divide_amount: i128 = divide_amounts[chunk_index_lo_first as usize];
    let extracted_chunk_0: i32 = (accumulator / divide_amount % chunk_modulo as i128) as i32;
    return extracted_chunk_0;
}
#[inline]
unsafe fn calculate_length_chunk_count(mut length: i64) -> i32 {
    let mut chunk_count: i32 = 0 as i32;
    let mut i: u64 = length as u64;
    while i != 0 {
        i >>= g_bits_per_length_chunk;
        chunk_count += 1;
    }
    if chunk_count == 0 as i32 {
        chunk_count = 1 as i32;
    }
    return chunk_count;
}

pub unsafe fn safe80_version() -> *const libc::c_char {
    return b"PROJECT_VERSION\0" as *const u8 as *const libc::c_char;
}

pub unsafe fn safe80_get_decoded_length(encoded_length: i64) -> i64 {
    if encoded_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let group_count: i64 = encoded_length / g_chunks_per_group as libc::c_long;
    let byte_count: i32 =
        g_chunk_to_byte_count[(encoded_length % g_chunks_per_group as libc::c_long) as usize];
    let result: i64 = group_count * g_bytes_per_group as libc::c_long + byte_count as libc::c_long;
    return result;
}

pub unsafe fn safe80_decode_feed(
    src_buffer_ptr: *mut *const u8,
    src_length: i64,
    dst_buffer_ptr: *mut *mut u8,
    dst_length: i64,
    stream_state: safe80_stream_state,
) -> safe80_status {
    if src_length < 0 as i32 as libc::c_long || dst_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH;
    }
    let mut src: *const u8 = *src_buffer_ptr;
    let mut dst: *mut u8 = *dst_buffer_ptr;
    let src_end: *const u8 = src.offset(src_length as isize);
    let dst_end: *const u8 = dst.offset(dst_length as isize);
    let mut last_src: *const u8 = src;
    let mut current_group_chunk_count: i32 = 0 as i32;
    let mut accumulator: i128 = 0 as i32 as i128;
    while src < src_end {
        let fresh0 = src;
        src = src.offset(1);
        let next_char: u8 = *fresh0;
        let next_chunk: u8 = g_encode_char_to_chunk[next_char as usize];
        if next_chunk as i32 == 0xfe as i32 {
            continue;
        }
        if next_chunk as i32 == 0xff as i32 {
            *src_buffer_ptr = src.offset(-(1 as i32 as isize));
            *dst_buffer_ptr = dst;
            return SAFE80_ERROR_INVALID_SOURCE_DATA;
        }
        accumulator = accumulate_chunk(accumulator, next_chunk);
        current_group_chunk_count += 1;
        if dst.offset(g_chunk_to_byte_count[current_group_chunk_count as usize] as isize)
            >= dst_end as *mut u8
        {
            break;
        }
        if current_group_chunk_count == g_chunks_per_group {
            let bytes_to_write: i32 = g_chunk_to_byte_count[current_group_chunk_count as usize];
            let mut i: i32 = bytes_to_write - 1 as i32;
            while i >= 0 as i32 {
                let fresh1 = dst;
                dst = dst.offset(1);
                *fresh1 = extract_byte_from_accumulator(accumulator, i) as u8;
                i -= 1;
            }
            current_group_chunk_count = 0 as i32;
            accumulator = 0 as i32 as i128;
            last_src = src;
        }
    }
    while src < src_end {
        if g_encode_char_to_chunk[*src as usize] as i32 != 0xfe as i32 {
            last_src = src;
            break;
        } else {
            src = src.offset(1);
        }
    }
    let mut src_is_at_end: bool =
        stream_state as libc::c_uint & SAFE80_SRC_IS_AT_END_OF_STREAM as i32 as libc::c_uint != 0
            && src >= src_end;
    let mut dst_is_at_end: bool =
        stream_state as libc::c_uint & SAFE80_DST_IS_AT_END_OF_STREAM as i32 as libc::c_uint != 0
            && dst.offset(g_chunk_to_byte_count[current_group_chunk_count as usize] as isize)
                >= dst_end as *mut u8;
    if current_group_chunk_count > 0 as i32
        && (src_is_at_end as i32 != 0 || dst_is_at_end as i32 != 0)
    {
        let bytes_to_write_0: i32 = g_chunk_to_byte_count[current_group_chunk_count as usize];
        let mut i_0: i32 = bytes_to_write_0 - 1 as i32;
        while i_0 >= 0 as i32 {
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = extract_byte_from_accumulator(accumulator, i_0) as u8;
            i_0 -= 1;
        }
        last_src = src;
        dst_is_at_end = stream_state as libc::c_uint
            & SAFE80_DST_IS_AT_END_OF_STREAM as i32 as libc::c_uint
            != 0
            && dst.offset(g_chunk_to_byte_count[current_group_chunk_count as usize] as isize)
                >= dst_end as *mut u8;
    }
    *src_buffer_ptr = last_src;
    *dst_buffer_ptr = dst;
    if src_is_at_end as i32 != 0 || dst_is_at_end as i32 != 0 {
        if stream_state as libc::c_uint & SAFE80_EXPECT_DST_STREAM_TO_END as i32 as libc::c_uint
            != 0
        {
            if dst_is_at_end {
                return SAFE80_STATUS_OK;
            } else {
                return SAFE80_ERROR_TRUNCATED_DATA;
            }
        } else if src_is_at_end {
            return SAFE80_STATUS_OK;
        } else {
            return SAFE80_ERROR_NOT_ENOUGH_ROOM;
        }
    }
    return SAFE80_STATUS_PARTIALLY_COMPLETE;
}

pub unsafe fn safe80_read_length_field(
    buffer: *const u8,
    buffer_length: i64,
    length: *mut i64,
) -> i64 {
    if buffer_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let max_pre_append_value: i64 = 9223372036854775807 as libc::c_long >> g_bits_per_length_chunk;
    let continuation_bit: i32 = (1 as i32) << g_bits_per_length_chunk;
    let max_chunk_value: i32 = continuation_bit - 1 as i32;
    let chunk_mask: i32 = continuation_bit - 1 as i32;
    let mut buffer_end: *const u8 = buffer.offset(buffer_length as isize);
    let mut value: i64 = 0 as i32 as i64;
    let mut next_chunk: i32 = 0 as i32;
    let mut src: *const u8 = buffer;
    while src < buffer_end {
        next_chunk = g_encode_char_to_chunk[*src as i32 as usize] as i32;
        if next_chunk == 0xfe as i32 {
            src = src.offset(1);
        } else {
            let chunk_value: i32 = next_chunk & !continuation_bit;
            if chunk_value > max_chunk_value {
                return SAFE80_ERROR_INVALID_SOURCE_DATA as i32 as i64;
            }
            if value > max_pre_append_value {
                return SAFE80_ERROR_INVALID_SOURCE_DATA as i32 as i64;
            }
            value = value << g_bits_per_length_chunk | (next_chunk & chunk_mask) as libc::c_long;
            src = src.offset(1);
            if next_chunk & continuation_bit == 0 {
                break;
            }
        }
    }
    if next_chunk & continuation_bit != 0 {
        return SAFE80_ERROR_UNTERMINATED_LENGTH_FIELD as i32 as i64;
    }
    *length = value;
    return src.offset_from(buffer) as libc::c_long;
}

pub unsafe fn safe80_decode(
    src_buffer: *const u8,
    src_length: i64,
    dst_buffer: *mut u8,
    dst_length: i64,
) -> i64 {
    if src_length < 0 as i32 as libc::c_long || dst_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let mut src: *const u8 = src_buffer;
    let mut dst: *mut u8 = dst_buffer;
    let status: safe80_status = safe80_decode_feed(
        &mut src,
        src_length,
        &mut dst,
        dst_length,
        (SAFE80_SRC_IS_AT_END_OF_STREAM as i32 | SAFE80_DST_IS_AT_END_OF_STREAM as i32)
            as safe80_stream_state,
    );
    if status as i32 != SAFE80_STATUS_OK as i32 {
        if status as i32 == SAFE80_STATUS_PARTIALLY_COMPLETE as i32 {
            return SAFE80_ERROR_NOT_ENOUGH_ROOM as i32 as i64;
        }
        return status as i64;
    }
    let mut decoded_byte_count: i64 = dst.offset_from(dst_buffer) as libc::c_long;
    return decoded_byte_count;
}

pub unsafe fn safe80l_decode(
    src_buffer: *const u8,
    src_length: i64,
    dst_buffer: *mut u8,
    dst_length: i64,
) -> i64 {
    if src_length < 0 as i32 as libc::c_long || dst_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let mut specified_length: i64 = 0 as i32 as i64;
    let bytes_used: i64 = safe80_read_length_field(src_buffer, src_length, &mut specified_length);
    if bytes_used < 0 as i32 as libc::c_long {
        return bytes_used;
    }
    let read_length: i64 = src_length - bytes_used;
    let mut src: *const u8 = src_buffer.offset(bytes_used as isize);
    let mut dst: *mut u8 = dst_buffer;
    let status: safe80_status = safe80_decode_feed(
        &mut src,
        read_length,
        &mut dst,
        specified_length,
        (SAFE80_SRC_IS_AT_END_OF_STREAM as i32
            | SAFE80_DST_IS_AT_END_OF_STREAM as i32
            | SAFE80_EXPECT_DST_STREAM_TO_END as i32) as safe80_stream_state,
    );
    if status as i32 != SAFE80_STATUS_OK as i32 {
        if status as i32 == SAFE80_STATUS_PARTIALLY_COMPLETE as i32 {
            return SAFE80_ERROR_NOT_ENOUGH_ROOM as i32 as i64;
        }
        return status as i64;
    }
    let mut decoded_byte_count: i64 = dst.offset_from(dst_buffer) as libc::c_long;
    if decoded_byte_count < specified_length {
        return SAFE80_ERROR_TRUNCATED_DATA as i32 as i64;
    }
    return decoded_byte_count;
}

pub unsafe fn safe80_get_encoded_length(decoded_length: i64, include_length_field: bool) -> i64 {
    if decoded_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let group_count: i64 = decoded_length / g_bytes_per_group as libc::c_long;
    let chunk_count: i32 =
        g_byte_to_chunk_count[(decoded_length % g_bytes_per_group as libc::c_long) as usize];
    let mut length_chunk_count: i32 = 0 as i32;
    if include_length_field {
        length_chunk_count = calculate_length_chunk_count(decoded_length);
    }
    return group_count * g_chunks_per_group as libc::c_long
        + chunk_count as libc::c_long
        + length_chunk_count as libc::c_long;
}

pub unsafe fn safe80_encode_feed(
    src_buffer_ptr: *mut *const u8,
    src_length: i64,
    dst_buffer_ptr: *mut *mut u8,
    dst_length: i64,
    is_end_of_data: bool,
) -> safe80_status {
    if src_length < 0 as i32 as libc::c_long || dst_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH;
    }
    let mut src: *const u8 = *src_buffer_ptr;
    let mut dst: *mut u8 = *dst_buffer_ptr;
    let src_end: *const u8 = src.offset(src_length as isize);
    let dst_end: *const u8 = dst.offset(dst_length as isize);
    let mut last_src: *const u8 = src;
    let mut current_group_byte_count: i32 = 0 as i32;
    let mut accumulator: i128 = 0 as i32 as i128;
    while src < src_end {
        let fresh3 = src;
        src = src.offset(1);
        let next_byte: u8 = *fresh3;
        accumulator = accumulate_byte(accumulator, next_byte);
        current_group_byte_count += 1;
        if current_group_byte_count == g_bytes_per_group {
            let mut chunks_to_write: i32 = g_byte_to_chunk_count[current_group_byte_count as usize];
            if dst.offset(chunks_to_write as isize) > dst_end as *mut u8 {
                *src_buffer_ptr = last_src;
                *dst_buffer_ptr = dst;
                return SAFE80_STATUS_PARTIALLY_COMPLETE;
            }
            let mut i: i32 = chunks_to_write - 1 as i32;
            while i >= 0 as i32 {
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 =
                    g_chunk_to_encode_char[extract_chunk_from_accumulator(accumulator, i) as usize];
                i -= 1;
            }
            current_group_byte_count = 0 as i32;
            accumulator = 0 as i32 as i128;
            last_src = src;
        }
    }
    if current_group_byte_count > 0 as i32 {
        if is_end_of_data {
            let mut chunks_to_write_0: i32 =
                g_byte_to_chunk_count[current_group_byte_count as usize];
            if dst.offset(chunks_to_write_0 as isize) > dst_end as *mut u8 {
                *src_buffer_ptr = last_src;
                *dst_buffer_ptr = dst;
                return SAFE80_STATUS_PARTIALLY_COMPLETE;
            }
            let mut i_0: i32 = chunks_to_write_0 - 1 as i32;
            while i_0 >= 0 as i32 {
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = g_chunk_to_encode_char
                    [extract_chunk_from_accumulator(accumulator, i_0) as usize];
                i_0 -= 1;
            }
        } else {
            src = src.offset(-(current_group_byte_count as isize));
        }
        last_src = src;
    }
    *src_buffer_ptr = last_src;
    *dst_buffer_ptr = dst;
    return SAFE80_STATUS_OK;
}

pub unsafe fn safe80_write_length_field(
    length: i64,
    dst_buffer: *mut u8,
    dst_buffer_length: i64,
) -> i64 {
    if dst_buffer_length < 0 as i32 as libc::c_long || length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let continuation_bit: i32 = (1 as i32) << g_bits_per_length_chunk;
    let chunk_mask: i32 = continuation_bit - 1 as i32;
    let mut chunk_count: i32 = 0 as i32;
    let mut i: u64 = length as u64;
    while i != 0 {
        i >>= g_bits_per_length_chunk;
        chunk_count += 1;
    }
    if chunk_count == 0 as i32 {
        chunk_count = 1 as i32;
    }
    if chunk_count as libc::c_long > dst_buffer_length {
        return SAFE80_ERROR_NOT_ENOUGH_ROOM as i32 as i64;
    }
    let mut dst: *mut u8 = dst_buffer;
    let mut shift_amount: i32 = chunk_count - 1 as i32;
    while shift_amount >= 0 as i32 {
        let should_continue: i32 = if shift_amount == 0 as i32 {
            0 as i32
        } else {
            continuation_bit
        };
        let chunk_value: i32 = ((length >> g_bits_per_length_chunk * shift_amount
            & chunk_mask as libc::c_long)
            + should_continue as libc::c_long) as i32;
        let next_char: u8 = g_chunk_to_encode_char[chunk_value as usize];
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = next_char;
        shift_amount -= 1;
    }
    return chunk_count as i64;
}

pub unsafe fn safe80_encode(
    src_buffer: *const u8,
    src_length: i64,
    dst_buffer: *mut u8,
    dst_length: i64,
) -> i64 {
    if src_length < 0 as i32 as libc::c_long || dst_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let mut src: *const u8 = src_buffer;
    let mut dst: *mut u8 = dst_buffer;
    let status: safe80_status =
        safe80_encode_feed(&mut src, src_length, &mut dst, dst_length, 1 as i32 != 0);
    if status as i32 != SAFE80_STATUS_OK as i32 {
        if status as i32 == SAFE80_STATUS_PARTIALLY_COMPLETE as i32 {
            return SAFE80_ERROR_NOT_ENOUGH_ROOM as i32 as i64;
        }
        return status as i64;
    }
    return dst.offset_from(dst_buffer) as libc::c_long;
}

pub unsafe fn safe80l_encode(
    src_buffer: *const u8,
    src_length: i64,
    dst_buffer: *mut u8,
    dst_length: i64,
) -> i64 {
    if src_length < 0 as i32 as libc::c_long || dst_length < 0 as i32 as libc::c_long {
        return SAFE80_ERROR_INVALID_LENGTH as i32 as i64;
    }
    let mut bytes_used: i64 = safe80_write_length_field(src_length, dst_buffer, dst_length);
    if bytes_used < 0 as i32 as libc::c_long {
        return bytes_used;
    }
    let mut src: *const u8 = src_buffer;
    let mut dst: *mut u8 = dst_buffer.offset(bytes_used as isize);
    let mut status: safe80_status =
        safe80_encode_feed(&mut src, src_length, &mut dst, dst_length, 1 as i32 != 0);
    if status as i32 != SAFE80_STATUS_OK as i32 {
        if status as i32 == SAFE80_STATUS_PARTIALLY_COMPLETE as i32 {
            return SAFE80_ERROR_NOT_ENOUGH_ROOM as i32 as i64;
        }
        return status as _;
    }
    return dst.offset_from(dst_buffer) as libc::c_long;
}
