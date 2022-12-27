import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import { useParams } from 'react-router'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import queryString from 'query-string'
import { ModalBack, ModalContent } from 'src/components/Modal'
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { Check } from '@material-ui/icons'
import { useEnvironment } from 'src/contexts/environment.context'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePollTxHash from 'src/hooks/usePollTxHash'
import { bonusBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'
import useQuestGetObjectives from 'src/hooks/useQuestGetObjectives'
import useQuestCurrentConfig from 'src/hooks/useQuestCurrentConfig'
import useQuestGetCompleted from 'src/hooks/useQuestGetCompleted'
import useQuestGetReward from 'src/hooks/useQuestGetReward'
import useCountdown from 'src/hooks/useCountdown'

const Form = styled.div`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 330px;
`

const CompleteQuest: React.FC = (): React.ReactElement => {
  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber?.toString() || ""
  const name = `xyz #${nameNumber}`
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  // query tasks
  const { objectives, count: objectiveCount } = useQuestGetObjectives()
  const { quest } = useQuestCurrentConfig()
  const { completedObjectives, count: completedCount } = useQuestGetCompleted(name)
  // denom and amount
  const { reward } = useQuestGetReward(name)

  const processedReward = reward ? parseInt(reward?.amount) / 1000000 : 0

  const canComplete = (objectiveCount === completedCount) && objectives !== undefined && completedObjectives !== undefined

  const {
    start_time: questStartTime,
    quest_duration_seconds: questDurationSeconds,
  } = quest || {}

  const { isComplete: questExpired } = useCountdown(
    parseInt(questStartTime || "") / 1000000, // nanoseconds
    parseInt(questDurationSeconds || "") * 1000, // seconds
  )

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx || questExpired) {
      queryClient.invalidateQueries(['marketplaceResourceAllowance'])
      queryClient.invalidateQueries(['resourceBalance'])
      queryClient.invalidateQueries(['questCurrentConfig'])
      queryClient.invalidateQueries(['questCompleted'])
      queryClient.invalidateQueries(['questObjectives'])
      queryClient.invalidateQueries(['questReward'])
      queryClient.invalidateQueries(['activityFeed'])
      queryClient.invalidateQueries(['questObjectiveCompletedCount'])
    }
  }, [tx, questExpired])

  const getDisabledResponse = () => {
    let response = "Transaction fee could not be estimated. Please try again later."
    if (questExpired && canComplete) {
      response = "Quest has expired."
    } else if (!canComplete) {
      response = "Objectives not yet completed."
    } else {
      response = "Reward can be claimed after quest has ended."
    }
    return response
  }

  // execute tasks
  const executes =
    connectedWallet !== undefined
      ? [
        new MsgExecuteContract(
          connectedWallet.walletAddress,
          environmentContext.QUEST_CONTRACT_ADDRESS,
          {
            complete_quest: {
              xyz_id: name,
            },
          },
        ),
      ]
      : []
  const { result: gasFee, gasDollars } = useGasEstimate(executes)
  const tryClaimDiscovery = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    if (gasFee.data === undefined) {
      throw new Error(
        `Transaction fee could not be estimated. Please try again later.`,
      )
    }
    return connectedWallet.post({
      msgs: executes,
      fee: gasFee.data,
    })
  }
  const mutation = useMutation<
    TxResult | undefined,
    { message: string },
    Record<string, unknown>
  >(tryClaimDiscovery, {
    onSuccess: async (data) => {
      if (data?.success) {
        setTxHash(data?.result.txhash)
      } else {
        window.alert('Transaction broadcast failed')
      }
    },
  })

  const submit = () => {
    mutation.mutate({})
  }

  return (
    <ModalContent>
      <ModalBack></ModalBack>
      {!!txHash && (
        <TxLoading
          tx={tx}
          txHash={txHash}
          successElement={
            <>
              {'Quest completed!'}
            </>
          }
        />
      )}
      {!txHash &&
        <Form>
          <div
            css={css`
                display: flex;
                flex-direction: column;
              `}
          >
            <H2
              css={css`
                align-self: center;
                ${bonusBackgroundText};
              `}
            >
              {'QUEST'}
            </H2>
          </div>
          <p>
            {`Current quest reward $${processedReward} UST.`}
          </p>
          <ul
              css={css`
                list-style-position: inside;
              `}
            >
              {
                objectives?.map((objective) => {
                  const { objective_id: objectiveId, goal } = objective
                  const { name: goalName } = goal || {}

                  const completed = completedObjectives?.filter((completed) => {
                    return completed.objective.objective_id === objectiveId
                  })

                  return (
                    <li css={css``}>
                      <span
                        css={css`
                          position: relative;
                          left: -10px;
                        `}
                      >
                        {`Objective ${objectiveId + 1}: ${goal !== undefined && goal !== null ? `${goalName}` : "Unrevealed objective"}`}
                        {completed && completed.length > 0 && (
                          <Check
                            css={css`
                              margin-left: 4px;
                              transform: scale(calc(16 / 24));
                            `}
                          />
                        )}
                      </span>
                    </li>
                  )
                })
              }
            </ul>

          <Ul>
            {gasDollars !== undefined && (
              <Li>{`Transaction fee: ${gasDollars} $UST`}</Li>
            )}
            {gasDollars === undefined && !gasFee.isFetching && (
              <Li
                css={css`
                  color: lightcoral;
                `}
              >
                {getDisabledResponse()}
              </Li>
            )}
          </Ul>
          {mutation.error && (
            <P
              css={css`
                  color: red;
                `}
            >
              {mutation.error.message}
            </P>
          )}
          {(
            <Button
              disabled={
                mutation.isLoading ||
                gasDollars === undefined ||
                gasFee.isFetching ||
                !canComplete ||
                (questExpired && !canComplete)
              }
              sizevariant={'large'}
              colorvariant={'secondary'}
              css={css`
                  margin-top: 15px;
                `}
              onClick={submit}
            >
              {'COMPLETE QUEST'}
            </Button>
          )}
        </Form>
      }
    </ModalContent>
  )
}

export default CompleteQuest
