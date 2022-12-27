import { useWallet } from '@terra-money/wallet-provider'
import React, { useMemo } from 'react'
import { Link } from 'react-router-dom'
import useCountdown from 'src/hooks/useCountdown'
import styled, { css, keyframes } from 'styled-components'

const bgAnimation = keyframes`
  from {
    background-position: 0% 0%;
  }
  to {
    background-position: 100% 0%;
  }
`
const FooterContainer = styled.footer`
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  grid-gap: 10px;
  padding: 15px 20px;
  width: 100%;
  position: fixed;
  bottom: 0;
  color: white;
  z-index: 1;
`

const Bottom = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  grid-gap: 20px;

  a {
    font-size: 14px;
    color: white;
    text-transform: uppercase;
  }
`
const Footer: React.FC = () => {
  const { status } = useWallet()

  const currentDate = useMemo(() => new Date().getTime(), [])
  const startTimeMilliseconds = Math.floor(currentDate)
  const taskDurationMilliseconds = 1000 * 5
  const { isComplete, countdownView } = useCountdown(
    startTimeMilliseconds,
    taskDurationMilliseconds,
  )

  return (
    <FooterContainer>
      <Bottom>
        <div
          css={css`
            display: flex;
            grid-gap: 20px;
            align-items: center;
          `}
        >
          {/* {status === WalletStatus.WALLET_CONNECTED && (
            <>
              {isComplete && (
                <Link
                  to={`?modal=${ModalTypes.ClaimBonus}`}
                  css={css`
              background-image: linear-gradient(
                to right,
                #fd1892,
                #b827fc,
                #2c90fc,
                #fd1892,
                #b827fc,
                #2c90fc,
                #fd1892,
                #b827fc,
                #2c90fc
              );
              -webkit-background-clip: text;
              background-clip: text;
              -webkit-text-fill-color: transparent;
              animation: ${bgAnimation} 3.5s linear infinite;
              background-size: 400% 100%;
            `}
                >
                  {'CLAIM BONUS.'}
                </Link>
              )}
              {!isComplete && (
                <p>{countdownView}</p>
              )}
              <p
                css={css`
              display: flex;
              align-items: center;
              grid-gap: 5px;
              font-size: 14px;
            `}
              >
                <img
                  src={Bonus}
                  css={css`
                width: 20px;
              `}
                />
                {'BONUS: 5'}
              </p>

            </>
          )} */}
        </div>
        <div
          css={css`
            display: flex;
            grid-gap: 20px;
          `}
        >
          <Link to={'/'}>{'HOME.'}</Link>
          <Link to={'/faq'}>{'FAQ.'}</Link>
          <a
            href={'https://twitter.com/collectxyznft'}
            target={'_blank'}
            rel={'noreferrer'}
          >
            {'TWITTER.'}
          </a>
        </div>
      </Bottom>
    </FooterContainer>
  )
}

export default Footer
