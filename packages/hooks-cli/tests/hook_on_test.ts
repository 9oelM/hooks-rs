import { assertEquals } from "jsr:@std/assert";
import { HookOnField } from "../hooks_builder/mod.ts";

Deno.test(`Should initialize default HookOn field`, () => {
  const hookOnField = new HookOnField().fromEmpty();
  const expected =
    `ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbfffff`;
  const actual = hookOnField.toHex();
  assertEquals(actual, expected);
});

Deno.test(`Should create HookOn field from a set of transaction types`, () => {
  const ttList: (keyof typeof XrplTransactionType)[] = [
    `ACCOUNT_DELETE`,
    `CHECK_CANCEL`,
    `PAYMENT`,
    `URITOKEN_MINT`,
  ];
  const hookOnField = new HookOnField().fromSet(new Set(ttList));
  const expected =
    `ffffffffffffffffffffffffffffffffffffffffffffffffffffdfffff9bfffe`;
  const actual = hookOnField.toHex();
  assertEquals(actual, expected);
});

Deno.test(`Should create HookOn field from hex string`, () => {
  const hex =
    `ffffffffffffffffffffffffffffffffffffffffffffffffffffdfffff9bfffe`;
  const hookOnField = new HookOnField().fromHex(hex);
  const expected =
    `ffffffffffffffffffffffffffffffffffffffffffffffffffffdfffff9bfffe`;
  const actual = hookOnField.toHex();
  assertEquals(actual, expected);
});
