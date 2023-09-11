use core::mem::MaybeUninit;

use crate::c;

use super::*;

/// Retrieve the data pointed to by a Hook State key and write it to an output buffer
/// The keys are always 32 bytes (unsigned 256 bit integer) and the values are variable
/// length with a maximum size determined by validator voting, at time of writing 128 bytes.
#[inline(always)]
pub fn state<const STATE_VALUE_LEN: usize>(key: &[u8]) -> Result<[u8; STATE_VALUE_LEN]>
where
{
    let mut uninitialized_buffer: [MaybeUninit<u8>; STATE_VALUE_LEN] = MaybeUninit::uninit_array();

    let initialized_buffer = unsafe {
        let result: Result<u64> = c::state(
            uninitialized_buffer.as_mut_ptr() as u32,
            STATE_VALUE_LEN as u32,
            key.as_ptr() as u32,
            key.len() as u32,
        )
        .into();

        match result {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        MaybeUninit::array_assume_init(uninitialized_buffer)
    };

    Ok(initialized_buffer)
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

    res.into()
}
