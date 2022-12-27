import { useWallet, WalletStatus } from '@terra-money/wallet-provider'
import React from 'react'
import { Link } from 'react-router-dom'
import { ModalTypes } from 'src/components/Modal'
import { useEasterEggs } from 'src/contexts/easterEggs.context'
import { mediaDown } from 'src/styles/breakpoints'
import { fonts } from 'src/styles/constants'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'
import ConnectWallet from './ConnectWallet'

const HeaderContainer = styled.header`
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 25px 0px;
  width: 900px;
  ${mediaDown('lg')(`width: 100%`)};
  ${mediaDown('lg')(`padding: 25px 10px`)};
`

const MainLogo = styled(Link)`
  font-size: 24px;
  font-family: ${fonts.highlight};
  color: white;
  border: 2px ridge white;
  border-image: linear-gradient(
    to bottom right,
    #b827fc 0%,
    #2c90fc 25%,
    #b8fd33 50%,
    #fec837 75%,
    #fd1892 100%
  );
  border-image-slice: 1;
  padding: 4px 12px;
  &:hover {
    color: white;
  }
`

const Left = styled.div`
  display: flex;
  align-items: center;
  grid-gap: 20px;
`

const Right = styled.div`
  display: flex;
  align-items: center;
  overflow: hidden;
  text-overflow: ellipsis;
  ${mediaDown('md')(`align-items: flex-start`)};
`

const Header: React.FC = () => {
  const { status } = useWallet()
  const { isBackgroundImageVisible, setIsBackgroundImageVisible } =
    useEasterEggs()

  return (
    <HeaderContainer>
      <Left>
        <MainLogo to={'/'}>{'xyz'}</MainLogo>
        <span
          role={'button'}
          onClick={() => {
            setIsBackgroundImageVisible((val) => !val)
          }}
          css={css`
            cursor: pointer;
            color: darkGrey;
            opacity: 0;
            transition: opacity 200ms ease-out;
            &:hover {
              opacity: 0.4;
            }
          `}
        >
          {isBackgroundImageVisible ? 'Hide stars?' : 'Show stars?'}
        </span>
      </Left>
      <Right>
        {status === WalletStatus.WALLET_CONNECTED && (
          <>
            <Link
              to={`?modal=${ModalTypes.Mint}`}
              css={css`
                ${completeTaskBackgroundText};
                margin-right: 30px;
                ${mediaDown('md')`
  margin-right: 10px;
  `};
              `}
            >
              {'MINT.'}
            </Link>
          </>
        )}
        <ConnectWallet css={css``} />
      </Right>
    </HeaderContainer>
  )
}

export default Header
