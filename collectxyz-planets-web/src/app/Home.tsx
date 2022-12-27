import { useWallet, WalletStatus } from '@terra-money/wallet-provider'
import React, { Fragment } from 'react'
import { Redirect, useLocation } from 'react-router'
import { Route, Switch } from 'react-router-dom'
import Collection from 'src/app/Collection'
import XyzDetail from 'src/app/XyzDetail'
import BackgroundPage from 'src/components/BackgroundPage'
import BonusBox from 'src/components/BonusBox'
import FadeInDelay from 'src/components/FadeInDelay'
import useActivityFeed from 'src/hooks/useActivityFeed'
import { mediaDown } from 'src/styles/breakpoints'
import styled, { css } from 'styled-components'

const Title = styled.div`
  display: flex;
  align-items: center;
  grid-gap: 10px;
  margin-bottom: 30px;
  color: white;
  align-self: stretch;
  justify-content: center;
  position: relative;
`

const Home: React.FC = (): React.ReactElement => {
  const { status } = useWallet()
  const location = useLocation()
  const feedItems = useActivityFeed()

  return (
    <BackgroundPage>
      <div
        css={css`
          position: fixed;
          top: 25px;
          left: 20px;
          bottom: 0px;
          display: flex;
          flex-direction: column;
          grid-gap: 8px;
          width: calc((100vw - 900px) / 2 - 50px);
          ${mediaDown('xl')`
  display: none;
  `};
          &:after {
            content: "";
            position: absolute;
            z-index: 1;
            bottom: 0;
            left: 0;
            pointer-events: none;
            background-image: linear-gradient(
              to bottom,
              rgba(0, 0, 0, 0),
              rgba(0, 0, 0, 1) 90%
            );
            width: 100%;
            height: 200px;
          }
        `}
      >
        {feedItems.map((item) => (
          <FadeInDelay key={item.id} index={0} totalCount={1}>
            <span
              key={item.id}
              css={css`
                font-size: 14px;
                line-height: 18px;
                color: darkGrey;
              `}
            >
              {item.description}
            </span>
          </FadeInDelay>
        ))}
      </div>
      {status === WalletStatus.WALLET_NOT_CONNECTED && (
        <>
          <p
            css={css`
              color: lightGrey;
              text-align: center;
            `}
          >
            {'Your wallet is not connected.'}
          </p>
        </>
      )}
      {status === WalletStatus.WALLET_CONNECTED && (
        <Fragment>
          <BonusBox></BonusBox>
          <Switch>
            <Route exact path={'/collection'}>
              <Collection></Collection>
            </Route>
            <Route path={'/xyz/:nameNumber'}>
              <XyzDetail></XyzDetail>
            </Route>
            <Route exact path={'/'}>
              <Redirect to={'/collection'}></Redirect>
            </Route>
          </Switch>
        </Fragment>
      )}
    </BackgroundPage>
  )
}

export default Home
