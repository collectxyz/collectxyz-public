import {
  ConnectType,
  useWallet,
  WalletStatus,
} from '@terra-money/wallet-provider'
import React from 'react'
import { useQueryClient } from 'react-query'
import { walletTruncated } from 'src/utils'
import styled, { css } from 'styled-components'

const ConnectWalletContainer = styled.div`
  display: flex;
  justify-content: space-between;
  flex-direction: column;
  overflow: hidden;
  text-overflow: ellipsis;
`

const ConnectWallet: React.FC = () => {
  const {
    status,
    connect,
    disconnect,
    wallets,
    network,
  } = useWallet()

  const queryClient = useQueryClient()

  const isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent)

  return <ConnectWalletContainer>
    {status === WalletStatus.WALLET_NOT_CONNECTED && (
      <>
        <span
          role={'button'}
          onClick={(): void => {
            queryClient.invalidateQueries()
            if (isMobile) {
              connect(ConnectType.WALLETCONNECT)
            } else {
              connect(ConnectType.CHROME_EXTENSION)
            }
          }}
          css={css`
            color: white;
            text-decoration: underline;
            cursor: pointer;
            display: flex;
            align-items: center;
            &:hover {
              color: lightGrey;
            }
          `}
        >
          {'Connect Wallet'}
        </span>
      </>
    )}
    {status === WalletStatus.WALLET_CONNECTED && (
      <>
        <span
          onClick={(): void => { disconnect() }}
          role={'button'}
          css={css`
            color: darkGrey;
            text-decoration: underline;
            white-space: nowrap;
            text-overflow: ellipsis;
            cursor: pointer;
            &:hover {
              color: lightGrey;
            }
          `}
        >
          {/* {'Disconnect terra3f8a...a8j1 (bombay)'} */}

          {`Disconnect ${walletTruncated(wallets[0].terraAddress)} (${network.name}).`}
        </span>
      </>
    )}

  </ConnectWalletContainer>
}

export default ConnectWallet
