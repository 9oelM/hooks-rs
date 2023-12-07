import { TypedObjectKeys } from "../types/mod.ts";
import { XrplTransactionType } from "./xrpl_transaction_type.ts";

export class HookOnField {
  private initialized = false;
  /**
   * includes 0x
   */
  private HOOK_ON_FIELD_HEX_REPRESENTATION_LENGTH = 66;
  private hookOn = BigInt(
    `0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbfffff`,
  );

  constructor() {}

  private xor(transactionType: keyof typeof XrplTransactionType) {
    const transactionTypeId = XrplTransactionType[transactionType];

    this.hookOn ^= 1n << BigInt(transactionTypeId);
  }

  // TODO: validate input hex string
  public fromHex(hex: string): HookOnField {
    if (this.initialized) {
      throw new Error("HookOnField already initialized");
    }

    const withPreceding0x = hex.startsWith("0x") ? hex : `0x${hex}`;

    if (
      withPreceding0x.length !== this.HOOK_ON_FIELD_HEX_REPRESENTATION_LENGTH
    ) {
      throw new Error(
        `HookOn hex representation must be ${this.HOOK_ON_FIELD_HEX_REPRESENTATION_LENGTH} characters long (including the preceding 0x) but got ${withPreceding0x.length}`,
      );
    }

    this.hookOn = BigInt(withPreceding0x);
    this.initialized = true;

    return this;
  }

  public fromSet(list: Set<keyof typeof XrplTransactionType>): HookOnField {
    if (this.initialized) {
      throw new Error("HookOnField already initialized");
    }

    for (const transactionType of list) {
      this.xor(transactionType);
    }
    this.initialized = true;

    return this;
  }

  /**
   * HookOnField is initialized to the default value
   */
  public fromEmpty(): HookOnField {
    if (this.initialized) {
      throw new Error("HookOnField already initialized");
    }

    this.initialized = true;

    return this;
  }

  public toHex(): string {
    if (!this.initialized) {
      throw new Error("HookOnField not initialized");
    }

    return `0x${this.hookOn.toString(16)}`;
  }

  public toList(): (keyof typeof XrplTransactionType)[] {
    if (!this.initialized) {
      throw new Error("HookOnField not initialized");
    }

    const list: (keyof typeof XrplTransactionType)[] = [];

    for (const transactionType of TypedObjectKeys(XrplTransactionType)) {
      if (this.hookOn & (1n << BigInt(XrplTransactionType[transactionType]))) {
        list.push(transactionType);
      }
    }

    return list;
  }
}
