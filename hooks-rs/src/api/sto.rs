use crate::c;

use super::*;

/// Index into a xrpld serialized object and return the location and length of a subfield
#[inline(always)]
pub fn sto_subfield(sto: &[u8], field_id: FieldId) -> Result<&[u8]> {
    let res = unsafe { c::sto_subfield(sto.as_ptr() as u32, sto.len() as u32, field_id as _) };

    let location = match res {
        res if res >= 0 => res,
        res => return Err(Error::from_code(res as _)),
    };

    Ok(&sto[range_from_location(location)])
}

/// Index into a xrpld serialized array and return the location and length of an index
#[inline(always)]
pub fn sto_subarray(sto: &[u8], array_id: u32) -> Result<&[u8]> {
    let res = unsafe { c::sto_subarray(sto.as_ptr() as u32, sto.len() as u32, array_id) };

    let location = match res {
        res if res >= 0 => res,
        res => return Err(Error::from_code(res as _)),
    };

    Ok(&sto[range_from_location(location)])
}

/// Emplace a field into an existing STObject at its canonical placement
#[inline(always)]
pub fn sto_emplace(
    sto_out: &mut [u8],
    sto_src: &[u8],
    field: &[u8],
    field_id: FieldId,
) -> Result<u64> {
    let res = unsafe {
        c::sto_emplace(
            sto_out.as_mut_ptr() as u32,
            sto_out.len() as u32,
            sto_src.as_ptr() as u32,
            sto_src.len() as u32,
            field.as_ptr() as u32,
            field.len() as u32,
            field_id as _,
        )
    };

    res.into()
}

/// Remove a field from an STObject
#[inline(always)]
pub fn sto_erase(sto_out: &mut [u8], sto_src: &[u8], field_id: FieldId) -> Result<u64> {
    let res = unsafe {
        c::sto_erase(
            sto_out.as_mut_ptr() as u32,
            sto_out.len() as u32,
            sto_src.as_ptr() as u32,
            sto_src.len() as u32,
            field_id as _,
        )
    };

    res.into()
}

/// Validate an STObject
#[inline(always)]
pub fn sto_validate(sto: &[u8]) -> bool {
    let res = buf_read(sto, c::sto_validate);

    match res {
        Ok(0) => false,
        Ok(1) => true,
        _ => false,
    }
}
