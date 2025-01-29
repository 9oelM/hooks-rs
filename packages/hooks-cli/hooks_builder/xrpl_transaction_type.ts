export const XrplTransactionType: Readonly<{
  readonly PAYMENT: 0;
  readonly ESCROW_CREATE: 1;
  readonly ESCROW_FINISH: 2;
  readonly ACCOUNT_SET: 3;
  readonly ESCROW_CANCEL: 4;
  readonly REGULAR_KEY_SET: 5;
  readonly OFFER_CREATE: 7;
  readonly OFFER_CANCEL: 8;
  readonly TICKET_CREATE: 10;
  readonly SIGNER_LIST_SET: 12;
  readonly PAYCHAN_CREATE: 13;
  readonly PAYCHAN_FUND: 14;
  readonly PAYCHAN_CLAIM: 15;
  readonly CHECK_CREATE: 16;
  readonly CHECK_CASH: 17;
  readonly CHECK_CANCEL: 18;
  readonly DEPOSIT_PREAUTH: 19;
  readonly TRUST_SET: 20;
  readonly ACCOUNT_DELETE: 21;
  readonly SET_HOOK: 22;
  readonly NFTOKEN_MINT: 25;
  readonly NFTOKEN_BURN: 26;
  readonly NFTOKEN_CREATE_OFFER: 27;
  readonly NFTOKEN_CANCEL_OFFER: 28;
  readonly NFTOKEN_ACCEPT_OFFER: 29;
  readonly URITOKEN_MINT: 45;
  readonly URITOKEN_BURN: 46;
  readonly URITOKEN_BUY: 47;
  readonly URITOKEN_CREATE_SELL_OFFER: 48;
  readonly URITOKEN_CANCEL_SELL_OFFER: 49;
  readonly IMPORT: 97;
  readonly CLAIM_REWARD: 98;
  readonly INVOKE: 99;
  readonly AMENDMENT: 100;
  readonly FEE: 101;
  readonly UNL_MODIFY: 102;
  readonly EMIT_FAILURE: 103;
}> = Object.freeze(
  {
    PAYMENT: 0,
    ESCROW_CREATE: 1,
    ESCROW_FINISH: 2,
    ACCOUNT_SET: 3,
    ESCROW_CANCEL: 4,
    REGULAR_KEY_SET: 5,
    OFFER_CREATE: 7,
    OFFER_CANCEL: 8,
    TICKET_CREATE: 10,
    SIGNER_LIST_SET: 12,
    PAYCHAN_CREATE: 13,
    PAYCHAN_FUND: 14,
    PAYCHAN_CLAIM: 15,
    CHECK_CREATE: 16,
    CHECK_CASH: 17,
    CHECK_CANCEL: 18,
    DEPOSIT_PREAUTH: 19,
    TRUST_SET: 20,
    ACCOUNT_DELETE: 21,
    SET_HOOK: 22,
    NFTOKEN_MINT: 25,
    NFTOKEN_BURN: 26,
    NFTOKEN_CREATE_OFFER: 27,
    NFTOKEN_CANCEL_OFFER: 28,
    NFTOKEN_ACCEPT_OFFER: 29,
    URITOKEN_MINT: 45,
    URITOKEN_BURN: 46,
    URITOKEN_BUY: 47,
    URITOKEN_CREATE_SELL_OFFER: 48,
    URITOKEN_CANCEL_SELL_OFFER: 49,
    IMPORT: 97,
    CLAIM_REWARD: 98,
    INVOKE: 99,
    AMENDMENT: 100,
    FEE: 101,
    UNL_MODIFY: 102,
    EMIT_FAILURE: 103,
  } as const,
);

export function isXrplTransactionType(
  transactionType: string,
): transactionType is keyof typeof XrplTransactionType {
  return transactionType in XrplTransactionType;
}
