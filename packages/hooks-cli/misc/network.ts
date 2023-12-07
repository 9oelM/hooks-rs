export enum Network {
  XahauTestnet = "XAHAU_TESTNET",
  XahauMainnet = "XAHAU_MAINNET",
}

const TESTNET_RPC_URL = "https://xahau-test.net";
const MAINNET_RPC_URL = "https://xahau.network";

export function getRpcUrl(network: Network) {
  switch (network) {
    case Network.XahauTestnet:
      return TESTNET_RPC_URL;
    case Network.XahauMainnet:
      return MAINNET_RPC_URL;
    default:
      throw new Error("Invalid network");
  }
}
