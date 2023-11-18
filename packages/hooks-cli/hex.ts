export class Hex {
  static isHex(value: string): boolean {
    return /^[0-9A-F]+$/iu.test(value)
  }
  
  static uint8ArrayToHexString(uint8Array: Uint8Array): string {
    let hexString = "";
    for (const byte of uint8Array) {
      const hex = byte.toString(16).padStart(2, "0");
      hexString += hex;
    }
    return hexString;
  }

  static stringToHexString(value: string): string {
    return this.uint8ArrayToHexString(new TextEncoder().encode(value))
  }
}