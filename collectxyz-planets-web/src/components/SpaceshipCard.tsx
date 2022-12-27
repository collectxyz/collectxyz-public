import { Check, Close } from '@material-ui/icons'
import React, { useMemo } from 'react'
import { Link } from 'react-router-dom'
import { TaskTypes } from 'src/app/StartTask'
import Bg from 'src/assets/images/bg.png'
import Bonus from 'src/assets/images/bonus.png'
import SpaceshipOne from 'src/assets/images/spaceshipOne.gif'
import { ModalTypes } from 'src/components/Modal'
import useCountdown from 'src/hooks/useCountdown'
import usePlanetsConfig from 'src/hooks/usePlanetsConfig'
import { TaskResponse } from 'src/models/task.models'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import { randIntRange } from 'src/utils'
import { css, keyframes } from 'styled-components'
import P from './P'

const bgAnimationGenerator = (initialX: number, initialY: number) => keyframes`
  from {
    background-position: ${initialX}% ${initialY}%;
  }
  to {
    background-position: ${initialX - 600}% ${initialY + 300}%;
  }
`

const shipAnimation = keyframes`
  0% {
    transform: rotate(60deg) translate(0px);
  }

  25% {
    transform: rotate(60deg) translate(5px);
  }

  75% {
    transform: rotate(60deg) translate(-5px);
  }

  100% {
    transform: rotate(60deg) translate(0px);
  }
`

interface SpaceshipCardProps {
  planetTask: TaskResponse
}
const SpaceshipCard: React.FC<SpaceshipCardProps> = (props) => {
  const initialX = useMemo(() => randIntRange(0, 100), [])
  const initialY = useMemo(() => randIntRange(0, 100), [])
  const bgAnimation = useMemo(
    () => bgAnimationGenerator(initialX, initialY),
    [],
  )

  const { data: planetsConfig } = usePlanetsConfig()

  const startTimeMilliseconds = Math.floor(
    parseInt(props.planetTask.start_time) / 1000000,
  )
  const taskDurationMilliseconds =
    1000 *
    (planetsConfig !== undefined ? planetsConfig.required_seconds : 60 * 100)
  const { isComplete, countdownView } = useCountdown(
    startTimeMilliseconds,
    taskDurationMilliseconds,
  )

  const { isComplete: hasExpired, countdownView: expirationCountdownView } = useCountdown(
    0,
    parseInt(props.planetTask.expires) / 1000000,
  )

  const containerCss = css`
    background: url("${Bg}");
    padding: 15px;
    animation: ${bgAnimation} 180s linear infinite;
    display: flex;
    flex-direction: column;
    width: 183px;
    box-shadow: 0 0 7px 3px lightgray;
    box-shadow: ${isComplete
    ? '0 0 7px 3px lightgray'
    : '0 0 3px 1px lightgray'};
    transition: box-shadow 200ms ease-out;
    &:hover {
      box-shadow: ${isComplete
    ? '0 0 4px 2px lightgray'
    : '0 0 3px 1px lightgray'};
    }
    position: relative;
  `

  return (
    <div css={containerCss}>
      {isComplete && (
        <Link
          to={`?modal=${ModalTypes.CompleteTask}&nameNumber=${
            props.planetTask.nft_token_id.split('#')[1]
          }&taskType=${TaskTypes.Exploration}`}
          css={css`
            position: absolute;
            top: 0px;
            left: 0px;
            bottom: 0px;
            right: 0px;
            z-index: 1;
          `}
        ></Link>
      )}
      <div
        css={css`
          position: absolute;
          top: 0px;
          left: 0px;
          bottom: 0px;
          right: 0px;
          opacity: ${isComplete ? 0 : 1};
          transition: opacity 200ms ease-out;
          background: ${'linear-gradient(to bottom right, rgba(0, 0, 0, 0.1), rgba(0, 0, 0, 0.5))'};
          z-index: 1;
          pointer-events: none;
        `}
      ></div>
      <div
        css={css`
          background-color: transparent;
          position: relative;
          display: flex;
          border: 2px ridge lightgray;
          aspect-ratio: 1/1;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          background-color: rgba(0, 255, 255, 0.05);
        `}
      >
        {props.planetTask.expected_boost > 0 && (
          <img
            src={Bonus}
            css={css`
              width: 20px;
              height: 20px;

              position: absolute;
              top: 5px;
              left: 5px;
            `}
          ></img>
        )}
        <P
          css={css`
            color: lightgray;
            position: absolute;
            top: 5px;
            right: 5px;
          `}
        >
          {props.planetTask.nft_token_id}
        </P>
        <img
          src={SpaceshipOne}
          css={css`
            height: 64px;
            width: 64px;
            transform: rotate(60deg);
            animation: ${shipAnimation} 4s linear infinite;
          `}
        />
      </div>
      <div
        css={css`
          display: flex;
          flex-direction: column;
          ${isComplete && !hasExpired ? completeTaskBackgroundText : undefined};
          // color: red;
            color: lightgray;
        `}
      >
        <P
          css={css`
            margin-top: 10px;
            display: flex;
            align-items: center;
            max-height: 16px;
          `}
        >
          {'EXPLORATION'}
          {isComplete && !hasExpired && (
            <Check
              css={css`
                transform: scale(calc(16 / 24));
              `}
            ></Check>
          )}
          {isComplete && hasExpired && (
            <Close
              css={css`
              color: red;
                transform: scale(calc(16 / 24));
              `}
            ></Close>
          )}
        </P>
        <P
          css={css`
            margin-top: 4px;
          `}
        >
          {
            isComplete
              ? hasExpired
                ? 'Expired'
                : 'Complete!'
              : `In progress...`
          }
        </P>
        <P
          css={css`
            margin-top: 5px;
            font-size: 12px;
          `}
        >
          {
            isComplete
              ? `(${expirationCountdownView} until expired)`
              : `(${countdownView} until complete)`

          }
        </P>
      </div>
    </div>
  )
}

export default SpaceshipCard
