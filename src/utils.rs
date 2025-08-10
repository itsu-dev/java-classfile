#[inline(always)]
pub fn read_u16(buffer: &[u8]) -> u16 {
    unsafe { *(buffer.as_ptr() as *const u16) }.to_le()
}

#[inline(always)]
pub fn read_u32(buffer: &[u8]) -> u32 {
    unsafe { *(buffer.as_ptr() as *const u32) }.to_le()
}

#[inline(always)]
pub fn read_i32(buffer: &[u8]) -> i32 {
    unsafe { *(buffer.as_ptr() as *const i32) }.to_le()
}

#[inline(always)]
pub fn read_f32(buffer: &[u8]) -> f32 {
    f32::from_bits(unsafe { *(buffer.as_ptr() as *const f32) }.to_bits().swap_bytes())
}

#[inline(always)]
pub fn read_i64(buffer: &[u8]) -> i64 {
    unsafe { *(buffer.as_ptr() as *const i64) }.to_le()
}

#[inline(always)]
pub fn read_f64(buffer: &[u8]) -> f64 {
    f64::from_bits(unsafe { *(buffer.as_ptr() as *const f64) }.to_bits().swap_bytes())
}

#[inline(always)]
pub fn read_str(buffer: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(buffer) }
}