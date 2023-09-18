use crate::api::*;

/// Comparable array of variables.
///
/// This can be used when manually calling `is_buffer_equal` to compare two arrays
/// is to be avoided.
///
/// Using ComparableArray over `is_buffer_equal` is generally preferred, since
/// it is more readable.
///
/// # Example
/// ```
/// const A: &[u8; 14] = b"same same same";
/// const B: &[u8; 14] = b"same same same";
///
/// const COMPARABLE_A: ComparableArray<u8, 14> = ComparableArray::new(A);
/// const COMPARABLE_B: ComparableArray<u8, 14> = ComparableArray::new(B);
///
/// if COMPARABLE_A != COMPARABLE_B {
///     rollback(b"arrays are not the same", -1);
/// }
/// ```
pub struct ComparableArray<'a, T, const N: usize>
where
    T: PartialEq,
{
    data: &'a [T; N],
}

/// Tests two buffers for equality
#[inline(always)]
pub fn is_buffer_equal<T: PartialEq>(buf_1: &[T], buf_2: &[T]) -> bool {
    let buf1_len = buf_1.len();

    if buf1_len != buf_2.len() {
        return false;
    };

    // guarded loop
    let mut i = 0;
    while {
        max_iter(buf1_len as u32 + 1);
        i < buf1_len
    } {
        if buf_1[i] != buf_2[i] {
            return false;
        }
        i += 1;
    }

    true
}

/// Zeroize a buffer
#[inline(always)]
pub fn buffer_zeroize<const GUARD_ID: u32>(buf: &mut [u8]) {
    let buf_len = buf.len();
    // guarded loop
    let mut i = 0;
    while {
        max_iter(buf_len as u32 + 1);
        i < buf_len
    } {
        buf[0] = 0;
        i += 1;
    }
}

impl<'a, T: PartialEq, const N: usize> ComparableArray<'a, T, N> {
    /// Create a new ComparableArray
    #[inline(always)]
    pub const fn new(data: &'a [T; N]) -> Self {
        Self { data }
    }
}

impl<T: PartialEq, const N: usize> AsRef<[T]> for ComparableArray<'_, T, N> {
    fn as_ref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T, const N: usize> PartialEq for ComparableArray<'_, T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        is_buffer_equal::<T>(self.as_ref(), other.as_ref())
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for ComparableArray<'a, T, N>
where
    T: PartialEq,
{
    fn from(data: &'a [T; N]) -> Self {
        Self::new(data)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use wasm_bindgen_test::*;

    // Due to some bug with wasm-pack or wasm-bindgen-test, this test does not compile.
    // and wasm-pack test --node does not pick up #[ignore] attribute. It's probably due to
    // the mutable-globals feature not being enabled for wasm-bindgen-test.
    // #[wasm_bindgen_test]
    // fn comparable_array_equal() {
    //     const A: ComparableArray<u8, 14> = ComparableArray::new(b"same same same");
    //     const B: ComparableArray<u8, 14> = ComparableArray::new(b"same same same");

    //     // assert_eq! requires ComparableArray to implement Debug trait,
    //     // but this is much simpler
    //     assert!(A == B);
    // }
}
