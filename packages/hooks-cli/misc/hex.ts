export function isHex(value: string): boolean {
  return /^[0-9A-F]+$/iu.test(value);
}

export function uint8ArrayToHexString(uint8Array: Uint8Array): string {
  let hexString = "";
  for (const byte of uint8Array) {
    const hex = byte.toString(16).padStart(2, "0");
    hexString += hex;
  }
  return hexString;
}

export function stringToHexString(value: string): string {
  return uint8ArrayToHexString(new TextEncoder().encode(value));
}
