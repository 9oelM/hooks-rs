use crate::c;

use super::*;

/// Retrieve the data pointed to by a Hook State key and write it to an output buffer
#[inline(always)]
pub fn state(data: &mut [u8], key: &[u8]) -> Result<u64> {
    buf_write_read(data, key, c::state)
}

/// Set the Hook State for a given key and value
#[inline(always)]
pub fn state_set(data: &[u8], key: &[u8]) -> Result<u64> {
    buf_2read(data, key, c::state_set)
}

/// Retrieve the data pointed to, on another account, by a Hook State key and write it to an output buffer
#[inline(always)]
pub fn state_foreign(data: &mut [u8], key: &[u8], namespace: &[u8], accid: &[u8]) -> Result<u64> {
    let res = unsafe {
        c::state_foreign(
            data.as_mut_ptr() as u32,
            data.len() as u32,
            key.as_ptr() as u32,
            key.len() as u32,
            namespace.as_ptr() as u32,
            namespace.len() as u32,
            accid.as_ptr() as u32,
            accid.len() as u32,
        )
    };

    result_u64(res)
}
