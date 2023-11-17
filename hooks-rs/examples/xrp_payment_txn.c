#include "hookapi.h"

int64_t cbak(uint32_t ctx) {
    _g(1, 1);

    return 0;
}

int64_t hook(uint32_t ctx)
{
    _g(1, 1);
    etxn_reserve(1);
    uint8_t otxn_account[20];
    if (otxn_field(otxn_account, 20, sfAccount) < 0) {
        rollback(SBUF("Failed to get account!"), 1);
    }
    TRACEHEX(otxn_account);
    uint8_t tx[270];
    PREPARE_PAYMENT_SIMPLE(
        tx, 1000, otxn_account, 0, 0
    )
    TRACEHEX(tx);
    uint8_t hash[32];
    if (emit(tx, 270, hash, 32) < 0)
    rollback("Failed to emit!", 15, 1);

    accept(hash, 32, 0);
}
