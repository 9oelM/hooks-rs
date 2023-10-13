# Writing your first transaction

<!-- toc -->

Writing a transaction on your own is quite a challenging process, as it requires some low-level understanding of how transactions are structured.

Let's go through the steps one by one by taking an example of an XRP payment transaction, which is already available [in the library](https://github.com/9oelM/hooks-rs/blob/main/src/transaction.rs) and [as an example](https://github.com/9oelM/hooks-rs/blob/main/examples/xrp_payment_txn.rs).

## Serialization format

Every XRPL transaction can be represented in binary format as well as JSON format. For this tutorial, we are most interested in the binary format, because we can only submit bytes to `emit` function to emit the transaction.

Here's [one example of a transaction in JSON and binary format, taken from XRPL documentation directly](https://xrpl.org/serialization.html#examples):

JSON

```json
{
  "Account": "rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys",
  "Expiration": 595640108,
  "Fee": "10",
  "Flags": 524288,
  "OfferSequence": 1752791,
  "Sequence": 1752792,
  "SigningPubKey": "03EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3",
  "TakerGets": "15000000000",
  "TakerPays": {
    "currency": "USD",
    "issuer": "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B",
    "value": "7072.8"
  },
  "TransactionType": "OfferCreate",
  "TxnSignature": "30440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C",
  "hash": "73734B611DDA23D3F5F62E20A173B78AB8406AC5015094DA53F53D39B9EDB06C"
}
```

Binary

```console
120007220008000024001ABED82A2380BF2C2019001ABED764D55920AC9391400000000000000000000000000055534400000000000A20B3C85F482532A9578DBB3950B85CA06594D165400000037E11D60068400000000000000A732103EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3744630440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C8114DD76483FACDEE26E60D8A586BB58D09F27045C46
```

## How to serialize your transaction

Look for the type of transaction you wish to implement from [XRPL transaction types](https://xrpl.org/transaction-types.html).

Let's say you want to implement [payment transaction](https://xrpl.org/payment.html), which is the most common transaction.

Then, identify [the required fields for transaction](https://xrpl.org/payment.html#payment-fields). In case of the payment transaction, only `Amount` and `Desitnation` are the required fields at the time of writing, so we will need to insert these fields in our transaction.

But in case you want to use other optional fields, you need to count them in too.

### Transaction common fields

Then, you will also need to insert [requied common fields](https://xrpl.org/transaction-common-fields.html#transaction-common-fields). At the time of writing, `Account`, `TransactionType`, `Fee`, and `Sequence` are the required common transaction fields. So we will need to serialize these into the resulting binary buffer as well.

### Sort the fields in canonical order

So far we've identified these fields:

| Field           | Internal type | Description                                                                                                                                                                              |
| --------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Amount          | Amount        | The amount of currency to deliver                                                                                                                                                        |
| Destination     | AccountID     | The unique address of the account receiving the payment                                                                                                                                  |
| Account         | AccountID     | The unique address of the account that initiated the transaction.                                                                                                                        |
| TransactionType | UInt16        | The type of transaction. Valid transaction types include: Payment, OfferCreate, TrustSet, and many others.                                                                               |
| Fee             | Amount        | Integer amount of XRP, in drops, to be destroyed as a cost for distributing this transaction to the network.                                                                             |
| Sequence        | UInt32        | The sequence number of the account sending the transaction. A transaction is only valid if the sequence number is exactly 1 greater than the previous transaction from the same account. |

Now let's learn how to sort these fields.

All fields in a transaction are sorted in a specific order based first on the field's type (specifically, a numeric "type code" assigned to each type), then on the field itself (a "field code").

Each internal type has its own type code. [You can find it from `SField.h`](https://github.com/XRPLF/rippled/blob/89780c8e4fd4d140fcb912cf2d0c01c1b260539e/src/ripple/protocol/SField.h#L54):

```c++
enum SerializedTypeID {
    // special types
    STI_UNKNOWN = -2,
    STI_NOTPRESENT = 0,

    // // types (common)
    STI_UINT16 = 1,
    STI_UINT32 = 2,
    STI_UINT64 = 3,
    STI_UINT128 = 4,
    STI_UINT256 = 5,
    STI_AMOUNT = 6,
    STI_VL = 7,
    STI_ACCOUNT = 8,
    // 9-13 are reserved
    STI_OBJECT = 14,
    STI_ARRAY = 15,

    // types (uncommon)
    STI_UINT8 = 16,
    STI_UINT160 = 17,
    STI_PATHSET = 18,
    STI_VECTOR256 = 19,
    STI_UINT96 = 20,
    STI_UINT192 = 21,
    STI_UINT384 = 22,
    STI_UINT512 = 23,
    STI_ISSUE = 24,

    // high level types
    // cannot be serialized inside other types
    STI_TRANSACTION = 10001,
    STI_LEDGERENTRY = 10002,
    STI_VALIDATION = 10003,
    STI_METADATA = 10004,
};
```

Then, you need to find [the field code from `SField.cpp`](https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L72-L266):

```c++
SField const sfInvalid     = make::one(&sfInvalid, -1);
SField const sfGeneric     = make::one(&sfGeneric, 0);
SField const sfLedgerEntry = make::one(&sfLedgerEntry, STI_LEDGERENTRY, 257, "LedgerEntry");
SField const sfTransaction = make::one(&sfTransaction, STI_TRANSACTION, 257, "Transaction");
SField const sfValidation  = make::one(&sfValidation,  STI_VALIDATION,  257, "Validation");
SField const sfMetadata    = make::one(&sfMetadata,    STI_METADATA,    257, "Metadata");
SField const sfHash        = make::one(&sfHash,        STI_HASH256,     257, "hash");
SField const sfIndex       = make::one(&sfIndex,       STI_HASH256,     258, "index");

// 8-bit integers
SF_U8 const sfCloseResolution   = make::one<SF_U8::type>(&sfCloseResolution,   STI_UINT8, 1, "CloseResolution");
SF_U8 const sfMethod            = make::one<SF_U8::type>(&sfMethod,            STI_UINT8, 2, "Method");
SF_U8 const sfTransactionResult = make::one<SF_U8::type>(&sfTransactionResult, STI_UINT8, 3, "TransactionResult");

// 8-bit integers (uncommon)
SF_U8 const sfTickSize          = make::one<SF_U8::type>(&sfTickSize,          STI_UINT8, 16, "TickSize");

// 16-bit integers
SF_U16 const sfLedgerEntryType = make::one<SF_U16::type>(&sfLedgerEntryType, STI_UINT16, 1, "LedgerEntryType", SField::sMD_Never);
SF_U16 const sfTransactionType = make::one<SF_U16::type>(&sfTransactionType, STI_UINT16, 2, "TransactionType");
SF_U16 const sfSignerWeight    = make::one<SF_U16::type>(&sfSignerWeight,    STI_UINT16, 3, "SignerWeight");

// 32-bit integers (common)
SF_U32 const sfFlags             = make::one<SF_U32::type>(&sfFlags,             STI_UINT32,  2, "Flags");
SF_U32 const sfSourceTag         = make::one<SF_U32::type>(&sfSourceTag,         STI_UINT32,  3, "SourceTag");
SF_U32 const sfSequence          = make::one<SF_U32::type>(&sfSequence,          STI_UINT32,  4, "Sequence");
SF_U32 const sfPreviousTxnLgrSeq = make::one<SF_U32::type>(&sfPreviousTxnLgrSeq, STI_UINT32,  5, "PreviousTxnLgrSeq", SField::sMD_DeleteFinal);
SF_U32 const sfLedgerSequence    = make::one<SF_U32::type>(&sfLedgerSequence,    STI_UINT32,  6, "LedgerSequence");
SF_U32 const sfCloseTime         = make::one<SF_U32::type>(&sfCloseTime,         STI_UINT32,  7, "CloseTime");
SF_U32 const sfParentCloseTime   = make::one<SF_U32::type>(&sfParentCloseTime,   STI_UINT32,  8, "ParentCloseTime");
SF_U32 const sfSigningTime       = make::one<SF_U32::type>(&sfSigningTime,       STI_UINT32,  9, "SigningTime");
SF_U32 const sfExpiration        = make::one<SF_U32::type>(&sfExpiration,        STI_UINT32, 10, "Expiration");
SF_U32 const sfTransferRate      = make::one<SF_U32::type>(&sfTransferRate,      STI_UINT32, 11, "TransferRate");
SF_U32 const sfWalletSize        = make::one<SF_U32::type>(&sfWalletSize,        STI_UINT32, 12, "WalletSize");
SF_U32 const sfOwnerCount        = make::one<SF_U32::type>(&sfOwnerCount,        STI_UINT32, 13, "OwnerCount");
SF_U32 const sfDestinationTag    = make::one<SF_U32::type>(&sfDestinationTag,    STI_UINT32, 14, "DestinationTag");

// 32-bit integers (uncommon)
SF_U32 const sfHighQualityIn       = make::one<SF_U32::type>(&sfHighQualityIn,       STI_UINT32, 16, "HighQualityIn");
SF_U32 const sfHighQualityOut      = make::one<SF_U32::type>(&sfHighQualityOut,      STI_UINT32, 17, "HighQualityOut");
SF_U32 const sfLowQualityIn        = make::one<SF_U32::type>(&sfLowQualityIn,        STI_UINT32, 18, "LowQualityIn");
SF_U32 const sfLowQualityOut       = make::one<SF_U32::type>(&sfLowQualityOut,       STI_UINT32, 19, "LowQualityOut");
SF_U32 const sfQualityIn           = make::one<SF_U32::type>(&sfQualityIn,           STI_UINT32, 20, "QualityIn");
SF_U32 const sfQualityOut          = make::one<SF_U32::type>(&sfQualityOut,          STI_UINT32, 21, "QualityOut");
SF_U32 const sfStampEscrow         = make::one<SF_U32::type>(&sfStampEscrow,         STI_UINT32, 22, "StampEscrow");
SF_U32 const sfBondAmount          = make::one<SF_U32::type>(&sfBondAmount,          STI_UINT32, 23, "BondAmount");
SF_U32 const sfLoadFee             = make::one<SF_U32::type>(&sfLoadFee,             STI_UINT32, 24, "LoadFee");

... and so on
```

For example, for `Amount` field, [the type code would be `STI_AMOUNT = 6`](https://github.com/XRPLF/rippled/blob/89780c8e4fd4d140fcb912cf2d0c01c1b260539e/src/ripple/protocol/SField.h#L65C8-L65C8) because it has an internal type of `Amount`, and [the field code would be `1`](https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L180).

So if we find all type codes and field codes for all fields and sort them in an ascending order, we get:

| Field           | Internal type | Description                                                                                                                                                                              | Type code | Field code |
| --------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- | ---------- |
| TransactionType | UInt16        | The type of transaction. Valid transaction types include: Payment, OfferCreate, TrustSet, and many others.                                                                               | 1         | 2          |
| Sequence        | UInt32        | The sequence number of the account sending the transaction. A transaction is only valid if the sequence number is exactly 1 greater than the previous transaction from the same account. | 2         | 4          |
| Amount          | Amount        | The amount of currency to deliver                                                                                                                                                        | 6         | 1          |
| Fee             | Amount        | Integer amount of XRP, in drops, to be destroyed as a cost for distributing this transaction to the network.                                                                             | 6         | 8          |
| Account         | AccountID     | The unique address of the account that initiated the transaction.                                                                                                                        | 8         | 1          |
| Destination     | AccountID     | The unique address of the account receiving the payment                                                                                                                                  | 8         | 3          |

So these fields will be inserted in the binary buffer in this exact order.

### Find field ids

Combine a field's type code and field code to get a field ID. This will be prefixed to each field in the buffer.

Although you can manually [compute the field ID by following the documentation](https://xrpl.org/serialization.html#field-ids), you can just use the constants that are already included in the library, as `FieldId::sfMyFieldName`. For example, a field ID of account is `FieldId::Account`.

### Prefix the length

Some types, specifically at the time of writing, `AccountID`, and `Blob`, require the binary field to be prefixed with the information about the field's own length. The length information will be prefixed _after_ the field ID.

[The rules are as simple as this](https://xrpl.org/serialization.html#length-prefixing):

- If the field contains 0 to 192 bytes of data, the first byte defines the length of the contents, then that many bytes of data follow immediately after the length byte.
- If the field contains 193 to 12480 bytes of data, the first two bytes indicate the length of the field with the following formula:
  ```
  193 + ((byte1 - 193) * 256) + byte2
  ```
- If the field contains 12481 to 918744 bytes of data, the first three bytes indicate the length of the field with the following formula:
  ```
  12481 + ((byte1 - 241) * 65536) + (byte2 * 256) + byte3
  ```

A length-prefixed field cannot contain more than 918744 bytes of data.

For example, the prefixed length for Account field will always be 20 in decimal or 0x14 in hex.

So let's complete the table with all information we have:

| Field           | Internal type | Description                                                                                                                                                                              | Type code | Field code | Field ID                              | Prefixed length |
| --------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- | ---------- | ------------------------------------- | --------------- |
| TransactionType | UInt16        | The type of transaction. Valid transaction types include: Payment, OfferCreate, TrustSet, and many others.                                                                               | 1         | 2          | 0x10002 or `FieldID::TransactionType` | none            |
| Sequence        | UInt32        | The sequence number of the account sending the transaction. A transaction is only valid if the sequence number is exactly 1 greater than the previous transaction from the same account. | 2         | 4          | 0x20004 or `FieldId::Sequence`        | none            |
| Amount          | Amount        | The amount of currency to deliver                                                                                                                                                        | 6         | 1          | 0x60001 or `FieldId::Amount`          | none            |
| Fee             | Amount        | Integer amount of XRP, in drops, to be destroyed as a cost for distributing this transaction to the network.                                                                             | 6         | 8          | 0x60008 or `FieldId::Fee`             | none            |
| Account         | AccountID     | The unique address of the account that initiated the transaction.                                                                                                                        | 8         | 1          | 0x80001 or `FieldId::Account`         | 0x14            |
| Destination     | AccountID     | The unique address of the account receiving the payment                                                                                                                                  | 8         | 3          | 0x80001 or `FieldId::Account`         | 0x14            |

Now we are really ready to put them into bytes.

## Using `TransactionBuilder`

hooks-rs provides a trait called `TransactionBuilder` and a struct called `TransactionBuffer`. It is recommended that you use these to build your own transaction. A detailed example can be found in the crate documentation.
