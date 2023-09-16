use super::*;
use core::mem::{MaybeUninit, self};

pub(crate) fn init_buffer_mut<const BUFFER_LEN: usize, F>(c_func: F) -> Result<[u8; BUFFER_LEN]>
where
    F: FnOnce(*mut MaybeUninit<u8>) -> Result<u64>,
{
    let mut uninitialized_buffer: [MaybeUninit<u8>; BUFFER_LEN] = MaybeUninit::uninit_array();
    let buffer: [u8; BUFFER_LEN] = unsafe {
        let result = c_func(uninitialized_buffer.as_mut_ptr());

        match result {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        mem::transmute::<_, [u8; BUFFER_LEN]>(
            uninitialized_buffer.as_ptr().cast::<[u8; BUFFER_LEN]>().read_volatile()
        )
    };

    Ok(buffer)
}
