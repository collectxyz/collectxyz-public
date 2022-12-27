import { Coin, LCDClient, WasmAPI } from '@terra-money/terra.js'
import { useConnectedWallet } from '@terra-money/wallet-provider'

// const gasPrices = await (await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices')).json()
// console.log(gasPrices)
const gasPrices: Record<string, string> = {
  uluna: '0.01133',
  usdr: '0.104938',
  uusd: '0.15',
  ukrw: '169.77',
  umnt: '428.571',
  ueur: '0.125',
  ucny: '0.98',
  ujpy: '16.37',
  ugbp: '0.11',
  uinr: '10.88',
  ucad: '0.19',
  uchf: '0.14',
  uaud: '0.19',
  usgd: '0.2',
  uthb: '4.62',
  usek: '1.25',
  unok: '1.25',
  udkk: '0.9',
  uidr: '2180.0',
  uphp: '7.6',
  uhkd: '1.17',
}
const gasPricesCoins = Object.keys(gasPrices).map(
  (token) => new Coin(token, gasPrices[token]),
)

export const useTerraClient = () => {
  const connectedWallet = useConnectedWallet()
  const terraClient =
    connectedWallet !== undefined
      ? new LCDClient({
        URL: connectedWallet.network.lcd,
        chainID: connectedWallet.network.chainID,
        gasPrices: gasPricesCoins,
        gasAdjustment: '1.3',
      })
      : undefined

  const api = terraClient ? new WasmAPI(terraClient.apiRequester) : undefined
  return { terraClient, api }
}
