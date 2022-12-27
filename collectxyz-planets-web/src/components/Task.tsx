import { Check } from '@material-ui/icons'
import React from 'react'
import { Link } from 'react-router-dom'
import Bg from 'src/assets/images/bg.png'
import Bonus from 'src/assets/images/bonus.png'
import useCountdown from 'src/hooks/useCountdown'
import { css, keyframes } from 'styled-components'
import P from './P'

const bgAnimation = keyframes`
  from {
    background-position: 0% 0%;
  }
  to {
    background-position: -600% -1200%;
  }
`

interface TaskProps {
  startMillis: number
  durationMillis: number
  expiryMillis: number
  to: string
  hasBoost?: boolean
  taskName: string
  image: React.ReactElement
  redBg?: boolean
  className?: string
}
const Task: React.FC<TaskProps> = (props) => {
  const startTimeMilliseconds = Math.floor(
    props.startMillis,
  )
  const { isComplete, countdownView } = useCountdown(
    startTimeMilliseconds,
    props.durationMillis,
  )

  const containerCss = css`
    background: url("${Bg}");
    padding: 10px 20px;
    animation: ${bgAnimation} 400s linear infinite;
    display: flex;
    align-items: center;
    height: 55px;
    width: 100%;
    box-shadow: ${isComplete
    ? '0 0 5px 2px lightgray'
    : '0 0 3px 1px lightgray'};
    transition: box-shadow 200ms ease-out;
    &:hover {
      box-shadow: ${isComplete
    ? '0 0 3px 1px lightgray'
    : '0 0 3px 1px lightgray'};
    }
    position: relative;
  `

  return (
    <div css={containerCss} className={props.className}>
      {isComplete && (
        <Link
          to={props.to}
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
      {props.hasBoost && (
        <img
          src={Bonus}
          css={css`
            width: 15px;
            height: 15px;
            position: absolute;
            top: 5px;
            left: 5px;
          `}
        ></img>
      )}
      <div
        css={css`
          position: absolute;
          top: 0px;
          left: 0px;
          bottom: 0px;
          right: 0px;
          opacity: ${isComplete ? 0.3 : 1};
          transition: opacity 200ms ease-out;
          background: ${'linear-gradient(to bottom right, rgba(0, 0, 0, 0.1), rgba(0, 0, 0, 0.6))'};
          z-index: 2;
          pointer-events: none;
        `}
      ></div>
      {props.redBg && (
        <div
          css={css`
          position: absolute;
          top: 0px;
          left: 0px;
          bottom: 0px;
          right: 0px;
          opacity: 0.3;
          transition: opacity 200ms ease-out;
          background: ${'linear-gradient(to bottom right, rgba(255, 0, 0, 0.1), rgba(180, 0, 0, 0.2))'};
          z-index: 1;
          pointer-events: none;
        `}
        ></div>
      )}
      {props.image}
      <P
        css={css`
          margin-left: 10px;
          display: flex;
          align-items: center;
          font-size: 16px;
        `}
      >
        {props.taskName}
      </P>
      {isComplete && (
        <Check
          css={css`
              margin-left: 10px;
              transform: scale(calc(24 / 24));
            `}
        ></Check>
      )}
      {!isComplete && (
        <P
          css={css`
            margin-left: 10px;
            font-size: 12px;
          `}
        >
          {`(${countdownView})`}
        </P>
      )}
    </div>
  )
}

export default Task
