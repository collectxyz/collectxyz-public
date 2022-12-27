import React, { useEffect } from 'react'
import { useQueryClient } from 'react-query'
import { Link } from 'react-router-dom'
import Quest from 'src/assets/images/quest.png'
import QuestCompleted from 'src/assets/images/quest_completed.png'
import { ModalTypes } from 'src/components/Modal'
import useCountdown from 'src/hooks/useCountdown'
import useDateFormat from 'src/hooks/useDateFormat'
import { CompleteObjective, Objective } from 'src/models/quest.models'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import { css } from 'styled-components'

interface ObjectiveCardProps {
  nameNumber: string
  objective: Objective
  completedObjective: CompleteObjective | null
}
const ObjectiveCard: React.FC<ObjectiveCardProps> = (props) => {
  const queryClient = useQueryClient()

  const containerCss = css`
    padding: 12px;
    display: flex;
    flex-direction: column;
    width: 190px;
    height: 225px;
    box-shadow: 0 0 5px 2px lightgray;
    transition: box-shadow 200ms ease-out;
    &:hover {
      box-shadow: 0 0 2px 1px lightgray;
    }
    position: relative;
    font-size: 12px;
    color: lightgray;
  `

  const { objective, nameNumber, completedObjective } = props

  const {
    objective_id: objectiveID,
    objective_start_time: objectiveStartTime,
    duration: objectiveDuration,
    multiplier,
    late_penalty: latePenalty,
    goal,
    possible_goals_info: possibleGoals,
  } = objective

  const {
    name: goalName,
    required_resources: requiredResources,
    xp_reward: xpReward,
  } = goal || {}

  const { isComplete: isObjExpired, countdownView } = useCountdown(
    0,
    parseInt(objectiveStartTime || '') / 1000000, // nanoseconds

    // parseInt(objectiveDuration || '') * 1000, // seconds
  )

  const { isComplete: isExpired, countdownView: lateCountdownView } = useCountdown(
    parseInt(objectiveStartTime || '') / 1000000, // nanoseconds
    parseInt(objectiveDuration || '') * 1000, // seconds
  )

  const switchCountdownView = (goal === null || goal === undefined ? countdownView : lateCountdownView)

  useEffect(() => {
    queryClient.invalidateQueries(['questCurrentConfig'])
    queryClient.invalidateQueries(['questCompleted'])
    queryClient.invalidateQueries(['questObjectives'])
    queryClient.invalidateQueries(['questReward'])
  }, [isObjExpired, isExpired])

  const countdownString = (goal === null || goal === undefined ? 'objective start' : 'cost increase')

  const resourceStringMapping: { [id: string]: string } = {
    'xyzROCK': 'ROCK',
    'xyzICE': 'ICE',
    'xyzMETAL': 'METAL',
    'xyzGAS': 'GAS',
    'xyzWATER': 'WATER',
    'xyzGEM': 'GEM',
    'xyzLIFE': 'LIFE',
  }

  return (
    <div css={containerCss}>
      <Link
        to={`?modal=${ModalTypes.ObjectiveDetail}&nameNumber=${nameNumber}&objectiveId=${objectiveID}`}
        css={css`
          position: absolute;
          top: 0px;
          left: 0px;
          bottom: 0px;
          right: 0px;
          z-index: 1;
        `}
      ></Link>
      <div
        css={css`
          display: flex;
          flex-direction: column;
        `}
      >
        <p
          css={css`
            display: flex;
            align-items: center;
            font-size: 16px;
            margin-bottom: 10px;
            ${completeTaskBackgroundText}
          `}
        >
          {goal !== undefined && goal !== null ? goalName : 'Unrevealed Objective'}
          {completedObjective !== null ? (
            <img
              src={QuestCompleted}
              css={css`
                width: 25px;
                height: 25px;
                position: absolute;
                top: 5px;
                right: 5px;
              `}
            />
          ) : (
            <img
              src={Quest}
              css={css`
                width: 25px;
                height: 25px;
                position: absolute;
                top: 5px;
                right: 5px;
              `}
            />
          )}
        </p>
        <p
          css={css`
            font-size: 12px;
            margin-bottom: 5px;
            color: lightgray;
          `}
        >
          {`Objective #${parseInt(objectiveID) + 1}`}
        </p>
        <div
          css={css`
            height: 1px;
            background-color: darkgray;
            width: 100%;
            margin-bottom: 10px;
          `}
        ></div>
        {
          goal !== undefined && goal !== null ? (
            <>
              <div
                css={css`
                  display: grid;
                  grid-template-columns: 1.5fr 1fr;
                  grid-gap: 8px;
                `}
              >
                <div
                  css={css`
                    display: flex;
                    flex-direction: column;
                  `}
                >
                  <p
                    css={css`
                      font-size: 14px;
                    `}
                  >
                    {'Requires:'}
                  </p>
                  <ul
                    css={css`
                      list-style-position: inside;
                    `}
                  >
                    {
                      requiredResources.map((goal) => {
                        return (
                          <li css={css``}>
                            <span
                              css={css`
                                position: relative;
                                left: -10px;
                              `}
                            >
                              {`${goal.required_amount} ${resourceStringMapping[goal.resource_id]}`}
                            </span>
                          </li>
                        )
                      })
                    }
                  </ul>
                </div>
                <div
                  css={css`
                    display: flex;
                    flex-direction: column;
                  `}
                >
                  <p
                    css={css`
                      font-size: 14px;
                    `}
                  >
                    {'Rewards:'}
                  </p>
                  <ul
                    css={css`
                      list-style-position: inside;
                    `}
                  >
                    <li css={css``}>
                      <span
                        css={css`
                          position: relative;
                          left: -10px;
                        `}
                      >
                        {`${xpReward} XP`}
                      </span>
                    </li>
                  </ul>
                </div>
              </div>
            </>
          ) : (
            <p
              css={css`
                font-size: 14px;
              `}
            >
              {'Objective has not started. Click to view possible goals.'}
            </p>
          )
        }
      </div>
      {
        completedObjective ? (
          <p
            css={css`
              display: flex;
              flex-direction: column;
              position: absolute;
              left: 12px;
              bottom: 12px;
            `}
          >
            {`objective completed ${useDateFormat(parseInt(completedObjective.completed_timestamp))}`}
          </p>
        ) : (
          <p
            css={css`
              display: flex;
              flex-direction: column;
              position: absolute;
              left: 12px;
              bottom: 12px;
            `}
          >
            {isExpired ? `late penalty applied (+${latePenalty})` : `(${switchCountdownView} until ${countdownString})`}
          </p>
        )
      }

    </div>
  )
}

export default ObjectiveCard
