enum Network {
  Testnet = "testnet",
  Mainnet = "mainnet",
}

const TESTNET_RPC_URL = "wss://xahau-test.net";
const MAINNET_RPC_URL = "wss://xahau.network";

export function getRpcUrl(network: Network) {
  switch (network) {
    case Network.Testnet:
      return TESTNET_RPC_URL;
    case Network.Mainnet:
      return MAINNET_RPC_URL;
    default:
      throw new Error("Invalid network");
  }
}
