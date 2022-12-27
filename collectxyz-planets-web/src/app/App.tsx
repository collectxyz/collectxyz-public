import queryString from 'query-string'
import React from 'react'
import { Route, Switch, useLocation } from 'react-router-dom'
import ClaimBonus from 'src/app/ClaimBonus'
import CompleteAllTasks from 'src/app/CompleteAllTasks'
import CompleteTask from 'src/app/CompleteTask'
import Faq from 'src/app/Faq'
import Home from 'src/app/Home'
import ListingDetail from 'src/app/ListingDetail'
import MakeListing from 'src/app/MakeListing'
import Mint from 'src/app/Mint'
import MoveCoordinates from 'src/app/MoveCoordinates'
import StartAllTasks from 'src/app/StartAllTasks'
import StartTask from 'src/app/StartTask'
import AboutQuest from 'src/app/AboutQuest'
import ObjectiveDetail from 'src/app/ObjectiveDetail'
import CompleteQuest from 'src/app/CompleteQuest'
import Bg from 'src/assets/images/bg4.png'
import Header from 'src/components/Header'
import Modal, { ModalTypes } from 'src/components/Modal'
import { useEasterEggs } from 'src/contexts/easterEggs.context'
import styled, { css } from 'styled-components'
import Footer from '../components/Footer'
import GlobalStyles from '../styles/globalStyles'

const AppContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100vw;
  height: 100vh;
  z-index: 1;
  position: relative;
`

const BgContainer = styled.div`
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: black;
  z-index: 0;
  background-image: url("${Bg}");
`

const BgMask = styled.div`
  transition: background-color 1000ms ease-out;
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
`

const App: React.FC = (): React.ReactElement => {
  const location = useLocation()
  const params = queryString.parse(location.search)
  const matchMint = params.modal === ModalTypes.Mint
  const matchStartTask = params.modal === ModalTypes.StartTask
  const matchCompleteTask = params.modal === ModalTypes.CompleteTask
  const matchClaimBonus = params.modal === ModalTypes.ClaimBonus
  const matchMoveCoordinates = params.modal === ModalTypes.MoveCoordinates
  const matchMakeListing = params.modal === ModalTypes.MakeListing
  const matchListingDetail = params.modal === ModalTypes.ListingDetail
  const matchStartAllTasks = params.modal === ModalTypes.StartAllTasks
  const matchCompleteAllTasks = params.modal === ModalTypes.CompleteAllTasks
  const matchAboutQuest = params.modal === ModalTypes.AboutQuest
  const matchObjectiveDetail = params.modal === ModalTypes.ObjectiveDetail
  const matchCompleteQuest = params.modal === ModalTypes.CompleteQuest
  const modalOpen = !!params.modal

  const {
    isBackgroundImageVisible,
  } = useEasterEggs()

  return (
    <>
      <GlobalStyles />
      <BgContainer>
        <BgMask css={css`
          background-color: rgba(0, 0, 0, ${isBackgroundImageVisible ? 0.55 : 1});
        `}></BgMask>
      </BgContainer>
      <AppContainer>
        <Header></Header>
        <Footer />
        {modalOpen && (
          <Modal>
            {matchMint && <Mint></Mint>}
            {matchStartTask && <StartTask></StartTask>}
            {matchCompleteTask && <CompleteTask></CompleteTask>}
            {matchClaimBonus && <ClaimBonus></ClaimBonus>}
            {matchMoveCoordinates && <MoveCoordinates></MoveCoordinates>}
            {matchMakeListing && <MakeListing></MakeListing>}
            {matchListingDetail && <ListingDetail></ListingDetail>}
            {matchStartAllTasks && <StartAllTasks></StartAllTasks>}
            {matchCompleteAllTasks && <CompleteAllTasks></CompleteAllTasks>}
            {matchAboutQuest && <AboutQuest></AboutQuest>}
            {matchObjectiveDetail && <ObjectiveDetail></ObjectiveDetail>}
            {matchCompleteQuest && <CompleteQuest></CompleteQuest>}
          </Modal>
        )}
        <Switch>
          <Route exact path={'/faq'}>
            <Faq />
          </Route>
          <Route path={'/'}>
            <Home />
          </Route>
        </Switch>
      </AppContainer>
    </>
  )
}

export default App
