use crate::c;

use super::*;

/// Convert a 20 byte Account ID to an r-address
#[inline(always)]
pub fn util_raddr(raddr_out: &mut [u8], accid: &[u8]) -> Result<u64> {
    buf_write_read(raddr_out, accid, c::util_raddr)
}

/// Convert an r-address into a 20 byte Account ID
#[inline(always)]
pub fn util_accid(accid_out: &mut [u8], raddr_in: &[u8]) -> Result<u64> {
    buf_write_read(accid_out, raddr_in, c::util_accid)
}

/// Verify a cryptographic signature
#[inline(always)]
pub fn util_verify(payload: &[u8], signature: &[u8], publickey: &[u8]) -> bool {
    let res = buf_3_read(payload, signature, publickey, c::util_verify);

    match res {
        Ok(0) => false,
        Ok(1) => true,
        _ => false,
    }
}

/// Compute an sha512-half over some data
#[inline(always)]
pub fn util_sha512h(hash_out: &mut [u8], data_in: &[u8]) -> Result<u64> {
    buf_write_read(hash_out, data_in, c::util_sha512h)
}

/// Compute a serialized keylet of a given type
#[inline(always)]
pub fn util_keylet(keylet: &mut [u8], keylet_type: KeyletType) -> Result<u64> {
    let write_ptr = keylet.as_mut_ptr() as _;
    let write_len = keylet.len() as _;

    match keylet_type {
        KeyletType::Hook(accid) => buf_read_and_zeroes(keylet, accid, c::KEYLET_HOOK),

        KeyletType::HookState(accid, statekey) => {
            buf_2_read_and_zeroes(keylet, accid, statekey, c::KEYLET_HOOK_STATE)
        }

        KeyletType::Account(accid) => buf_read_and_zeroes(keylet, accid, c::KEYLET_ACCOUNT),

        KeyletType::Amendments => all_zeroes(keylet, c::KEYLET_AMENDMENTS),

        KeyletType::Child(key) => buf_read_and_zeroes(keylet, key, c::KEYLET_CHILD),

        KeyletType::Skip(opt) => match opt {
            None => all_zeroes(keylet, c::KEYLET_SKIP),

            Some((ledger_index, num)) => {
                let res = unsafe {
                    c::util_keylet(
                        write_ptr,
                        write_len,
                        c::KEYLET_SKIP,
                        ledger_index,
                        num,
                        0,
                        0,
                        0,
                        0,
                    )
                };

                res.into()
            }
        },

        KeyletType::Fees => all_zeroes(keylet, c::KEYLET_FEES),

        KeyletType::NegativeUnl => all_zeroes(keylet, c::KEYLET_NEGATIVE_UNL),

        KeyletType::Line(accid_high, accid_low, currency_code) => {
            let res = unsafe {
                c::util_keylet(
                    write_ptr,
                    write_len,
                    c::KEYLET_LINE,
                    accid_high.as_ptr() as _,
                    accid_high.len() as _,
                    accid_low.as_ptr() as _,
                    accid_low.len() as _,
                    currency_code.as_ptr() as _,
                    currency_code.len() as _,
                )
            };

            res.into()
        }

        KeyletType::Offer(accid, num) => buf_read_and_1_arg(keylet, accid, num, c::KEYLET_OFFER),

        KeyletType::Quality(serialized_keylet, bits_high, bits_low) => buf_read_and_2_args(
            keylet,
            serialized_keylet,
            bits_high,
            bits_low,
            c::KEYLET_QUALITY,
        ),

        KeyletType::EmittedDir => all_zeroes(keylet, c::KEYLET_EMITTED_DIR),

        KeyletType::Signers(accid) => buf_read_and_zeroes(keylet, accid, c::KEYLET_SIGNERS),

        KeyletType::Check(accid, num) => buf_read_and_1_arg(keylet, accid, num, c::KEYLET_CHECK),

        KeyletType::DepositPreauth(accid_1, accid_2) => {
            buf_2_read_and_zeroes(keylet, accid_1, accid_2, c::KEYLET_DEPOSIT_PREAUTH)
        }

        KeyletType::Unchecked(key) => buf_read_and_zeroes(keylet, key, c::KEYLET_UNCHECKED),

        KeyletType::OwnerDir(accid) => buf_read_and_zeroes(keylet, accid, c::KEYLET_OWNER_DIR),

        KeyletType::Page(key, bits_high, bits_low) => {
            buf_read_and_2_args(keylet, key, bits_high, bits_low, c::KEYLET_PAGE)
        }

        KeyletType::Escrow(accid, num) => buf_read_and_1_arg(keylet, accid, num, c::KEYLET_ESCROW),

        KeyletType::Paychan(accid_1, accid_2, num) => {
            let res = unsafe {
                c::util_keylet(
                    write_ptr,
                    write_len,
                    c::KEYLET_PAYCHAN,
                    accid_1.as_ptr() as _,
                    accid_1.len() as _,
                    accid_2.as_ptr() as _,
                    accid_2.len() as _,
                    num,
                    0,
                )
            };

            res.into()
        }

        KeyletType::Emitted(key) => buf_read_and_zeroes(keylet, key, c::KEYLET_EMITTED),
    }
}
