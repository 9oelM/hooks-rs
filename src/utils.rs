use ::core::mem::MaybeUninit;
use ::core::ptr;

pub(crate) fn uninit_buffer<T: Default, const N: usize>() -> [T; N] {
    let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
    unsafe {
        let slots = &mut *array.as_mut_ptr();
        for slot in slots {
            ptr::write(slot, T::default());
        }
        array.assume_init()
    }
}
