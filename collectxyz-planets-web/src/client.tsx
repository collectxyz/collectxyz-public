import { NetworkInfo, WalletProvider } from '@terra-money/wallet-provider'
import 'core-js/stable'
import React from 'react'
import { CookiesProvider } from 'react-cookie'
import ReactDOM from 'react-dom'
import {
  QueryClient,
  QueryClientProvider,
} from 'react-query'
import { BrowserRouter } from 'react-router-dom'
import 'regenerator-runtime/runtime'
import App from 'src/app/App'
import { CurrentTimeDateContextProvider } from 'src/contexts/currentTimeDate.context'
import { EasterEggsContextProvider } from 'src/contexts/easterEggs.context'
import { EnvironmentContextProvider } from 'src/contexts/environment.context'
import type { } from 'styled-components/cssprop'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
      staleTime: Infinity,
    },
  },
})

declare global {
  interface Window {
    CAPTCHA_PUBLIC_KEY: string
    CAPTCHA_URL: string
    XYZ_CONTRACT_ADDRESS: string
    PLANETS_CONTRACT_ADDRESS: string
    RANDOMNESS_CONTRACT_ADDRESS: string
    BONUS_TOKEN_CONTRACT_ADDRESS: string
    RESOURCE_GATHERING_CONTRACT_ADDRESS: string
    XYZ_ROCK_CONTRACT_ADDRESS: string
    XYZ_METAL_CONTRACT_ADDRESS: string
    XYZ_ICE_CONTRACT_ADDRESS: string
    XYZ_GAS_CONTRACT_ADDRESS: string
    XYZ_WATER_CONTRACT_ADDRESS: string
    XYZ_GEM_CONTRACT_ADDRESS: string
    XYZ_LIFE_CONTRACT_ADDRESS: string
    XYZ_XP_CONTRACT_ADDRESS: string
    MARKETPLACE_CONTRACT_ADDRESS: string
    QUEST_CONTRACT_ADDRESS: string
  }
}

const mainnet: NetworkInfo = {
  name: 'mainnet',
  chainID: 'columbus-5',
  lcd: 'https://lcd.terra.dev',
}
const testnet: NetworkInfo = {
  name: 'testnet',
  chainID: 'bombay-12',
  lcd: 'https://bombay-lcd.terra.dev',
}
const walletConnectChainIds: Record<number, NetworkInfo> = {
  0: testnet,
  1: mainnet,
}

const render = (): void => {
  const html = (
    <QueryClientProvider client={queryClient} >
      <WalletProvider
        defaultNetwork={mainnet}
        walletConnectChainIds={walletConnectChainIds}
      >
        <EnvironmentContextProvider>
          <EasterEggsContextProvider>
            <CurrentTimeDateContextProvider>
              <CookiesProvider>
                <BrowserRouter>
                  <App />
                </BrowserRouter>
              </CookiesProvider>
            </CurrentTimeDateContextProvider>
          </EasterEggsContextProvider>
        </EnvironmentContextProvider>
      </WalletProvider>
    </QueryClientProvider>
  )
  ReactDOM.render(html, document.getElementById('content'))
}

if (module.hot) { // hot module replacement of front-end
  module.hot.accept(['app/App'], (): void => {
    console.log('hot module replacement')
    render()
  })
}

render()
