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
  public from_hex(hex: string): void {
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

    this.hookOn = BigInt(`0x${hex}`);
    this.initialized = true;
  }

  public from_list(list: (keyof typeof XrplTransactionType)[]): void {
    if (this.initialized) {
      throw new Error("HookOnField already initialized");
    }

    for (const transactionType of list) {
      this.xor(transactionType);
    }
    this.initialized = true;
  }

  /**
   * HookOnField is initialized to the default value
   */
  public from_empty(): void {
    if (this.initialized) {
      throw new Error("HookOnField already initialized");
    }

    this.initialized = true;
  }

  public to_hex(): string {
    if (!this.initialized) {
      throw new Error("HookOnField not initialized");
    }

    return this.hookOn.toString(16);
  }

  public to_list(): (keyof typeof XrplTransactionType)[] {
    if (!this.initialized) {
      throw new Error("HookOnField not initialized");
    }

    const list: (keyof typeof XrplTransactionType)[] = [];

    for (const transactionType of TypedObjectKeys(XrplTransactionType)) {
      if (this.hookOn & (1n << BigInt(XrplTransactionType[transactionType]))) {
        list.push(transactionType as keyof typeof XrplTransactionType);
      }
    }

    return list;
  }
}
