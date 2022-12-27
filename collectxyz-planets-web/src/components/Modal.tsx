import { ArrowBack } from '@material-ui/icons'
import React from 'react'
import { useHistory } from 'react-router-dom'
import Bg from 'src/assets/images/bg4.png'
import useNavigation from 'src/hooks/useNavigation'
import { mediaDown } from 'src/styles/breakpoints'
import styled, { css, keyframes } from 'styled-components'

export enum ModalTypes {
  Mint = 'mint',
  StartTask = 'startTask',
  StartAllTasks = 'startAllTasks',
  CompleteTask = 'claimTask',
  CompleteAllTasks = 'completeAllTasks',
  ClaimBonus = 'claimBonus',
  XyzDetail = 'xyzDetail',
  MoveCoordinates = 'moveCoordinates',
  MakeListing = 'makeListing',
  ListingDetail = 'listingDetail',
  ObjectiveDetail = 'objectiveDetail',
  AboutQuest = 'aboutQuest',
  CompleteQuest = 'completeQuest',
}

const fadeIn = keyframes`
  from {
    opacity: 0;
    transform: scale(0.99);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
`
const ModalContainer = styled.div`
  display: flex;
  position: fixed;
  flex-direction: column;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  align-items: center;
  overflow: auto;
  z-index: 3;
  padding: 80px 0px;
  animation: ${fadeIn} 300ms ease-out;
  ${mediaDown('md')`
  padding: 80px 10px;
  `};
`

const Background = styled.div`
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  opacity: 0.85;
  background-color: black;
  z-index: 0;
`

const bgAnimation = keyframes`
  from {
    background-position: ${0}% ${0}%;
  }
  to {
    background-position: ${0 - 1200}% ${0 + 900}%;
  }
`
export const ModalContent = styled.div`
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  color: white;
  box-shadow: 0 0 7px 2px lightgray;
  padding: 40px 0px;
  width: 500px;
  z-index: 1;
  background: url("${Bg}");
  animation: ${bgAnimation} 720s linear infinite;
  ${mediaDown('md')`
    width: 100%;
    padding: 40px 15px;
  `};
`

export const ModalBack: React.FC = (props) => {
  const history = useHistory()

  const goBack = () => {
    (history as unknown as any).goBack()
  }
  return (
    <ArrowBack
      role={'button'}
      onClick={goBack}
      css={css`
        cursor: pointer;
        position: absolute;
        top: 5px;
        left: 5px;
      `}
    ></ArrowBack>
  )
}
interface ModalProps {}
const Modal: React.FC<ModalProps> = (props) => {
  const { closeModal } = useNavigation()
  return (
    <ModalContainer>
      <Background
        onClick={() => {
          closeModal()
        }}
      ></Background>
      {props.children}
    </ModalContainer>
  )
}

export default Modal
