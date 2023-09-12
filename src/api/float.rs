use core::{ops::{Mul, Sub, Add, Div, Neg}, cmp::Ordering};

use crate::c;

use super::*;

/// XFL floating point number
#[derive(Clone, Copy)]
pub struct XFL(i64);

/// Multiply an XFL floating point by a non-XFL numerator and denominator
#[inline(always)]
pub fn float_mulratio(
    float1: XFL,
    round_up: bool,
    numerator: u32,
    denominator: u32,
) -> Result<XFL> {
    let res = unsafe { c::float_mulratio(float1.0, round_up as _, numerator, denominator) };

    res.into()
}

/// Negate an XFL floating point number
#[inline(always)]
pub fn float_negate(float: XFL) -> Result<XFL> {
    let res = unsafe { c::float_negate(float.0) };

    res.into()
}

/// XFL compare mode
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum XFLCompareMode {
    Less,
    Equal,
    Greater,
    NotEqual,
    LessOrEqual,
    GreaterOrEqual,
}

/// Perform a comparison on two XFL floating point numbers
#[inline(always)]
pub fn float_compare(float1: XFL, float2: XFL, mode: XFLCompareMode) -> Result<bool> {
    let mode = match mode {
        XFLCompareMode::Less => c::COMPARE_LESS,
        XFLCompareMode::Equal => c::COMPARE_EQUAL,
        XFLCompareMode::Greater => c::COMPARE_GREATER,
        XFLCompareMode::NotEqual => c::COMPARE_LESS | c::COMPARE_GREATER,
        XFLCompareMode::LessOrEqual => c::COMPARE_LESS | c::COMPARE_EQUAL,
        XFLCompareMode::GreaterOrEqual => c::COMPARE_GREATER | c::COMPARE_EQUAL,
    };

    let res = unsafe { c::float_compare(float1.0, float2.0, mode) };

    match res {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::from_code(res as _)),
    }
}

/// Add two XFL numbers together
#[inline(always)]
pub fn float_sum(float1: XFL, float2: XFL) -> Result<XFL> {
    let res = unsafe { c::float_sum(float1.0, float2.0) };

    res.into()
}

/// Output an XFL as a serialized object
#[inline(always)]
pub fn float_sto(
    amount: &mut [u8],
    currency_code: &[u8],
    issuer_accid: &[u8],
    float: XFL,
    field_code: FieldId,
) -> Result<u64> {
    let res = unsafe {
        c::float_sto(
            amount.as_mut_ptr() as _,
            amount.len() as _,
            currency_code.as_ptr() as _,
            currency_code.len() as _,
            issuer_accid.as_ptr() as _,
            issuer_accid.len() as _,
            float.0,
            field_code as _,
        )
    };

    res.into()
}

/// Read a serialized amount into an XFL
#[inline(always)]
pub fn float_sto_set(sto_xfl: &[u8]) -> Result<XFL> {
    let res = unsafe { c::float_sto_set(sto_xfl.as_ptr() as _, sto_xfl.len() as _) };

    res.into()
}

/// Divide one by an XFL floating point number
#[inline(always)]
pub fn float_invert(float: XFL) -> Result<XFL> {
    let res = unsafe { c::float_invert(float.0) };

    res.into()
}

/// Divide an XFL by another XFL floating point number
#[inline(always)]
pub fn float_divide(float1: XFL, float2: XFL) -> Result<XFL> {
    let res = unsafe { c::float_divide(float1.0, float2.0) };

    res.into()
}

/// Return the number 1 represented in an XFL enclosing number
#[inline(always)]
pub fn float_one() -> XFL {
    XFL(unsafe { c::float_one() })
}

/// Get the exponent of an XFL enclosing number
#[inline(always)]
pub fn float_exponent(float: XFL) -> i64 {
    unsafe { c::float_exponent(float.0) }
}

/// Get the mantissa of an XFL enclosing number
#[inline(always)]
pub fn float_mantissa(float: XFL) -> i64 {
    unsafe { c::float_mantissa(float.0) }
}

/// Get the sign of an XFL enclosing number
#[inline(always)]
pub fn float_sign(float: XFL) -> Result<bool> {
    match unsafe { c::float_sign(float.0) } {
        0 => Ok(false),
        1 => Ok(true),
        res => Err(Error::from_code(res as _)),
    }
}

/// Set the exponent of an XFL enclosing number
#[inline(always)]
pub fn float_exponent_set(float: XFL, exponent: i32) -> Result<XFL> {
    let res = unsafe { c::float_exponent_set(float.0, exponent) };

    res.into()
}

/// Set the mantissa of an XFL enclosing number
#[inline(always)]
pub fn float_mantissa_set(float: XFL, mantissa: i64) -> Result<XFL> {
    let res = unsafe { c::float_mantissa_set(float.0, mantissa) };

    res.into()
}

/// Set the sign of an XFL enclosing number
#[inline(always)]
pub fn float_sign_set(float: XFL, sign: bool) -> XFL {
    XFL(unsafe { c::float_sign_set(float.0, sign as _) })
}

/// Convert an XFL floating point into an integer (floor)
#[inline(always)]
pub fn float_int(float: XFL, decimal_places: u32, absolute: bool) -> Result<u64> {
    let res = unsafe { c::float_int(float.0, decimal_places, absolute as _) };

    res.into()
}

impl XFL {
    /// Create a new XFL number from an exponent and mantissa
    pub fn new(
        exponent: i32,
        mantissa: i64,
    ) -> Result<XFL> {
        unsafe { c::float_set(exponent, mantissa) }.into()
    }

    pub fn from_sto(
        serialized_xfl: &[u8; XFL_LEN],
    ) -> Result<XFL> {
        unsafe { c::float_sto_set(serialized_xfl.as_ptr() as _, XFL_LEN as _) }.into()
    }

    /// Multiply an XFL floating point by a non-XFL numerator and denominator
    pub fn mulratio(&self, round_up: bool, numerator: u32, denominator: u32) -> Result<XFL> {
        unsafe { c::float_mulratio(self.0, round_up as _, numerator, denominator) }.into()
    }
}

impl Add for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn add(self, other: XFL) -> Self::Output {
        float_sum(self, other)
    }
}

impl Sub for XFL {
    type Output = Result<XFL>;
    
    #[inline(always)]
    fn sub(self, other: XFL) -> Self::Output {
        let rhs = match float_negate(other) {
            Ok(rhs) => rhs,
            Err(e) => return Err(e),
        };
        float_sum(self, rhs)
    }
}

impl Mul for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn mul(self, other: XFL) -> Self::Output {
        unsafe { c::float_multiply(self.0, other.0) }.into()    
    }
}

impl Div for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn div(self, other: XFL) -> Self::Output {
        float_divide(self, other)
    }
}

impl Neg for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        float_negate(self)
    }
}

impl PartialEq for XFL {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        let res = unsafe { c::float_compare( self.0, other.0, c::COMPARE_EQUAL) };

        match res {
            1 => true,
            // This is based on the invariant that the arguments to eq function
            // are all valid XFL numbers.
            _ => false,
        }
    }
}

impl PartialOrd for XFL {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match float_compare(*self, *other, XFLCompareMode::Equal) {
            Ok(true) => Some(Ordering::Equal),
            Ok(false) => {
                // This is because float_compare cannot return an ordering at once
                match float_compare(*self, *other, XFLCompareMode::Less) {
                    Ok(true) => Some(Ordering::Less),
                    Ok(false) => {
                        match float_compare(*self, *other, XFLCompareMode::Greater) {
                            Ok(true) => Some(Ordering::Greater),
                            Ok(false) => None,
                            Err(_) => None,
                        }
                    },
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }
}
