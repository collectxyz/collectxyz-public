import React from 'react'
import { TaskTypes } from 'src/app/StartTask'
import SpaceshipOne from 'src/assets/images/spaceshipOne.gif'
import { ModalTypes } from 'src/components/Modal'
import Task from 'src/components/Task'
import usePlanetsConfig from 'src/hooks/usePlanetsConfig'
import { TaskResponse } from 'src/models/task.models'
import { css, keyframes } from 'styled-components'

const shipAnimation = keyframes`
  0% {
    transform: rotate(60deg) translate(0px);
  }

  25% {
    transform: rotate(60deg) translate(2px);
  }

  75% {
    transform: rotate(60deg) translate(-2px);
  }

  100% {
    transform: rotate(60deg) translate(0px);
  }
`

interface ExplorationTaskProps {
  task: TaskResponse
  className?: string
}
const ExplorationTask: React.FC<ExplorationTaskProps> = (props) => {
  const { data: planetsConfig } = usePlanetsConfig()
  const taskDurationMilliseconds =
    1000 *
    (planetsConfig !== undefined ? planetsConfig.required_seconds : 60 * 100)
  return (
    <Task
      className={props.className}
      startMillis={Math.floor(parseInt(props.task.start_time) / 1000000)}
      durationMillis={taskDurationMilliseconds}
      expiryMillis={Math.floor(parseInt(props.task.expires) / 1000000)}
      to={`?modal=${ModalTypes.CompleteTask}&nameNumber=${
        props.task.nft_token_id.split('#')[1]
      }&taskType=${TaskTypes.Exploration}`}
      hasBoost={props.task.expected_boost > 0}
      taskName={'EXPLORATION'}
      image={ <img
        src={SpaceshipOne}
        css={css`
          height: 35px;
          width: 35px;
          transform: rotate(60deg);
          animation: ${shipAnimation} 4s linear infinite;
        `}
      />
      }
    />
  )
}

export default ExplorationTask
