import React from 'react'
import { TaskTypes } from 'src/app/StartTask'
import SpaceshipOne from 'src/assets/images/spaceshipThree.gif'
import { ModalTypes } from 'src/components/Modal'
import Task from 'src/components/Task'
import useResourceGatheringConfig from 'src/hooks/useResourceGatheringConfig'
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

interface ResourceGatheringTaskProps {
  task: TaskResponse
}
const ResourceGatheringTask: React.FC<ResourceGatheringTaskProps> = (props) => {
  const { data: resourceGatheringConfig } = useResourceGatheringConfig()
  const taskDurationMilliseconds =
    1000 *
    (resourceGatheringConfig !== undefined ? resourceGatheringConfig.gather_task_duration_seconds : 60 * 100)
  return (
    <Task
      startMillis={Math.floor(parseInt(props.task.start_time) / 1000000)}
      durationMillis={taskDurationMilliseconds}
      expiryMillis={Math.floor(parseInt(props.task.expires) / 1000000)}
      to={`?modal=${ModalTypes.CompleteTask}&nameNumber=${
        props.task.nft_token_id.split('#')[1]
      }&taskType=${TaskTypes.ResourceGathering}`}
      hasBoost={props.task.expected_boost > 0}
      taskName={'GATHERING'}
      image={ <img
        src={SpaceshipOne}
        css={css`
          height: 40px;
          width: 40px;
          transform: rotate(60deg);
          animation: ${shipAnimation} 4s linear infinite;
        `}
      />
      }
      redBg
    />
  )
}

export default ResourceGatheringTask
