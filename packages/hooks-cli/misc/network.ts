export enum Network {
  XahauTestnet = "XAHAU_TESTNET",
  XahauMainnet = "XAHAU_MAINNET",
}

const TESTNET_WS_RPC_URL = "ws://xahau-test.net";
const TESTNET_HTTP_RPC_URL = "https://xahau-test.net";
const MAINNET_WS_RPC_URL = "wss://xahau.network";
const MAINNET_HTTP_RPC_URL = "https://xahau.network";

export function getRpcUrl(network: Network, ws?: boolean) {
  switch (network) {
    case Network.XahauTestnet:
      if (ws) {
        return TESTNET_WS_RPC_URL;
      }
      return TESTNET_HTTP_RPC_URL;
    case Network.XahauMainnet:
      if (ws) {
        return MAINNET_WS_RPC_URL;
      }
      return MAINNET_HTTP_RPC_URL;
    default:
      throw new Error("Invalid network");
  }
}
