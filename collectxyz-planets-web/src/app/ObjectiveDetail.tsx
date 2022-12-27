import { MsgExecuteContract } from '@terra-money/terra.js'
import React, { useState, useEffect } from 'react'
import queryString from 'query-string'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import { useMutation, useQueryClient } from 'react-query'
import H2 from 'src/components/H2'
import Button from 'src/components/Button'
import { ModalBack, ModalContent } from 'src/components/Modal'
import Ul from 'src/components/Ul'
import Li from 'src/components/Li'
import { completeTaskBackgroundText, bonusBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'
import useQuestGetObjectives from 'src/hooks/useQuestGetObjectives'
import useQuestGetCompleted from 'src/hooks/useQuestGetCompleted'
import { useEnvironment } from 'src/contexts/environment.context'
import useCountdown from 'src/hooks/useCountdown'
import useResourceBalance from 'src/hooks/useResourceBalance'
import useXyzQuestResourceAllowance from 'src/hooks/useXyzQuestResourceAllowance'
import useGasEstimate from 'src/hooks/useGasEstimate'
import useQuestCurrentConfig from 'src/hooks/useQuestCurrentConfig'
import usePollTxHash from 'src/hooks/usePollTxHash'
import TxLoading from 'src/components/TxLoading'
import useDateFormat from 'src/hooks/useDateFormat'
import useGetObjectiveCompletedCount from 'src/hooks/useGetObjectiveCompletedCount'
import Quest from 'src/assets/images/quest.png'
import QuestCompleted from 'src/assets/images/quest_completed.png'

const Container = styled.div`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 320px;
`
const ObjectiveDetail: React.FC = (): React.ReactElement => {
  useEffect(() => {
    queryClient.invalidateQueries(['questObjectiveCompletedCount'])
  }, [])

  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber?.toString() || ""
  const name = `xyz #${nameNumber}`
  const objectiveId = params.objectiveId?.toString() || "0"

  // get and parse objective
  const { objectives: allObjectives } = useQuestGetObjectives()
  const { completedObjectives } = useQuestGetCompleted(name)
  const { quest } = useQuestCurrentConfig()
  const { globalObjectiveCount } = useGetObjectiveCompletedCount(parseInt(objectiveId))

  const temporary = allObjectives?.filter((objective) => objective.objective_id.toString() === objectiveId)
  const objective = temporary && temporary.length > 0 && temporary[0] ? temporary[0] : null
  const {
    objective_id: confirmedObjectiveId,
    objective_start_time: objectiveStartTime,
    duration,
    goal,
    late_penalty: latePenalty,
    desc,
    multiplier,
    possible_goals_info: possibleGoals,
  } = objective || {}

  const {
    name: goalName,
    required_resources: requiredResources,
    xp_reward: xpReward,
  } = goal || {}

  const {
    start_time: questStartTime,
    quest_duration_seconds: questDurationSeconds,
  } = quest || {}

  const { isComplete: isExpired, countdownView } = useCountdown(
    parseInt(objectiveStartTime || "") / 1000000, // nanoseconds
    parseInt(duration || "") * 1000, // seconds
  )

  const { isComplete: questExpired } = useCountdown(
    parseInt(questStartTime || "") / 1000000, // nanoseconds
    parseInt(questDurationSeconds || "") * 1000, // seconds
  )

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx || isExpired || questExpired) {
      queryClient.invalidateQueries(['marketplaceResourceAllowance'])
      queryClient.invalidateQueries(['resourceBalance'])
      queryClient.invalidateQueries(['questCurrentConfig'])
      queryClient.invalidateQueries(['questCompleted'])
      queryClient.invalidateQueries(['questObjectives'])
      queryClient.invalidateQueries(['questReward'])
      queryClient.invalidateQueries(['activityFeed'])
      queryClient.invalidateQueries(['questObjectiveCompletedCount'])
    }
  }, [tx, questExpired, isExpired])

  // get resource balances  
  const { amountNumber: rockAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
  )
  const { amountNumber: metalAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
  )
  const { amountNumber: iceAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
  )
  const { amountNumber: gasAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_GAS_CONTRACT_ADDRESS,
  )
  const { amountNumber: waterAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_WATER_CONTRACT_ADDRESS,
  )
  const { amountNumber: gemAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_GEM_CONTRACT_ADDRESS,
  )
  const { amountNumber: lifeAmountNumber } = useResourceBalance(
    name,
    environmentContext.XYZ_LIFE_CONTRACT_ADDRESS,
  )

  // get resource allowances
  const { result: xyzRockMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
    )
  const { result: xyzMetalMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
    )
  const { result: xyzIceMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
    )
  const { result: xyzGasMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_GAS_CONTRACT_ADDRESS,
    )
  const { result: xyzWaterMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_WATER_CONTRACT_ADDRESS,
    )
  const { result: xyzGemMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_GEM_CONTRACT_ADDRESS,
    )
  const { result: xyzLifeMarketplaceAllowanceResult } =
    useXyzQuestResourceAllowance(
      name,
      environmentContext.XYZ_LIFE_CONTRACT_ADDRESS,
    )

  // create resource object
  const resources = [
    {
      id: 'xyzROCK',
      balance: rockAmountNumber || 0,
      allowance: parseInt(
        xyzRockMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzMETAL',
      balance: metalAmountNumber || 0,
      allowance: parseInt(
        xyzMetalMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzICE',
      balance: iceAmountNumber || 0,
      allowance: parseInt(
        xyzIceMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzGAS',
      balance: gasAmountNumber || 0,
      allowance: parseInt(
        xyzGasMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_GAS_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzWATER',
      balance: waterAmountNumber || 0,
      allowance: parseInt(
        xyzWaterMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_WATER_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzGEM',
      balance: gemAmountNumber || 0,
      allowance: parseInt(
        xyzGemMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_GEM_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzLIFE',
      balance: lifeAmountNumber || 0,
      allowance: parseInt(
        xyzLifeMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_LIFE_CONTRACT_ADDRESS,
    },
  ]

  const getResourceBalance = (resourceId: string) => { 
    const temp = resources.filter((resource) => resource.id === resourceId)
    return (temp && temp.length > 0 ? temp[0].balance : '0')
  }

  const notEnoughResources = requiredResources?.map(reqResource => {
    const balance = resources.filter(resource => resource.id === reqResource.resource_id).at(0)?.balance || 0
    return balance >= parseInt(reqResource.required_amount)
  }).includes(false)

  // check if objective is already completed
  const matchedCompletedObjectives = completedObjectives?.filter(completed => {
    return completed?.objective?.objective_id === confirmedObjectiveId
  })
  const isObjectiveCompleted = matchedCompletedObjectives && matchedCompletedObjectives.length > 0

  const allowanceExecutes =
    connectedWallet !== undefined
      ? resources
        .filter(
          (resource) => resource.allowance < (resource.balance || 0) * 1000000,
        )
        .map(
          (resource) =>
            new MsgExecuteContract(
              connectedWallet.walletAddress,
              resource.address,
              {
                increase_allowance: {
                  owner_xyz_id: name,
                  spender: environmentContext.QUEST_CONTRACT_ADDRESS,
                  amount: `${1000000 * 1000000000}`, // 1 billion allowance
                  expires: {
                    never: {},
                  },
                },
              },
            ),
        )
      : []
  const execute =
    connectedWallet !== undefined
      ? [
        new MsgExecuteContract(
          connectedWallet.walletAddress,
          environmentContext.QUEST_CONTRACT_ADDRESS,
          {
            complete_objective: {
              xyz_id: name,
              objective_id: confirmedObjectiveId
            },
          },
        ),
      ]
      : []
  const executes = [...allowanceExecutes, ...execute]
  const { result: gasFee, gasDollars } = useGasEstimate(executes)

  const completeObjectiveMutation = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    if (gasFee.data === undefined) {
      throw new Error(
        `Transaction fee could not be estimated. Please verify you have enough resources.`,
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
  >(completeObjectiveMutation, {
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

  const resourceStringMapping:{ [id: string]: string } = {
    "xyzROCK": "ROCK",
    "xyzICE": "ICE",
    "xyzMETAL": "METAL",
    "xyzGAS": "GAS",
    "xyzWATER": "WATER",
    "xyzGEM": "GEM",
    "xyzLIFE": "LIFE",
  }

  const resourceGoalStringMapping:{ [id: string]: string } = {
    "rock_weighting": "ROCK",
    "ice_weighting": "ICE",
    "metal_weighting": "METAL",
    "gas_weighting": "GAS",
    "water_weighting": "WATER",
    "gem_weighting": "GEM",
    "life_weighting": "LIFE",
  }

  const getDisabledResponse = () => {
    let response = "Transaction fee could not be estimated."
    if (questExpired) {
      response = "Quest has expired."
    } else if (notEnoughResources) {
      response = "Not enough resources."
    } else if (isObjectiveCompleted) {
      response = `Objective completed ${matchedCompletedObjectives?.at(0)?.completed_timestamp}.`
    }
    return response
  }

  return (
    <ModalContent css={css``}>
      <ModalBack></ModalBack>
      {!!txHash && (
        <TxLoading
          tx={tx}
          txHash={txHash}
          successElement={
            <>
              {'Objective completed!'}
            </>
          }
        />
      )}
      {!txHash &&
        <>
          <Container>
            <div
              css={css`
                display: flex;
                flex-direction: column;
              `}
            >
            <H2
              css={css`
                display: flex;
                align-items: center;
                margin-bottom: 2px;
                ${bonusBackgroundText}
              `}
            >
              {goal !== undefined && goal !== null ? goalName : "Unrevealed goal"}
            </H2>
            <p
              css={css`
                font-size: 15px;
                margin-bottom: 10px;
                color: lightgray;
              `}
            >
              {`Objective #${confirmedObjectiveId !== undefined ? parseInt(confirmedObjectiveId) + 1 : -1}`}
              {isObjectiveCompleted ? (
                <img
                  src={QuestCompleted}
                  css={css`
                    width: 25px;
                    height: 25px;
                    transform: translateY(5px);
                  `}
                /> 
              ) : (
                <img 
                  src={Quest}
                  css={css`
                    width: 25px;
                    height: 25px;
                    transform: translateY(5px);
                  `}
                />
              )}
            </p>
            <p css={css`margin-bottom: 10px`}>
              {desc}
            </p>
            {
              <p 
                css={css`
                  margin-bottom: 10px;
                  font-size: 0.95em;
                `}
              >
                {`Objective completed by ${globalObjectiveCount} XYZs.`}
              </p>
            }
            <div
              css={css`
                height: 1px;
                background-color: darkgray;
                width: 100%;
                margin-bottom: 10px;
              `}
            ></div>
            <div>
              <div
                css={css`
                  display: flex;
                  flex-direction: column;
                `}
              >
                {
                  goal !== undefined && goal !== null ? (
                    <>
                      <p
                        css={css`
                          font-size: 16px;
                          margin-bottom: 6px;
                        `}
                      >
                        {'Resources Required:'}
                      </p>
                      <ul
                        css={css`
                          list-style-position: inside;
                        `}
                      >
                        {
                          requiredResources?.map((goal) => {
                            const resourceBalance = getResourceBalance(goal.resource_id)
                            const requiredAmount = goal.required_amount
                            return (
                              <li css={css``}>
                                <span
                                  css={css`
                                    position: relative;
                                    left: -10px;
                                  `}
                                >
                                  <span css={css`
                                    color: ${resourceBalance >= requiredAmount ? '#2c90fc' : '#fd1892'}
                                  `}>
                                    {`${resourceBalance}/${requiredAmount}`}
                                  </span>
                                  {` ${resourceStringMapping[goal.resource_id]}`}
                                </span>
                              </li>
                            )
                          })
                        }
                      </ul>
                    </>
                  ) : (
                    <>
                      <p
                          css={css`
                            font-size: 16px;
                            margin-bottom: 6px;
                          `}
                        >
                          {'Possible objectives:'}
                      </p>
                     {
                        possibleGoals?.map((goal, index) => {
                          const {
                            name,
                            xp_amount: xpWeighting,
                          } = goal
                          let filtered = Object.fromEntries(
                            Object.entries(goal).filter(([k, v]) => v !== null && k !== "name" && k !== "xp_amount")
                          )
                          const resourceString = Object.entries(filtered).map(([k, v]) => `${v}% ${resourceGoalStringMapping[k]}`).join(", ")

                          return (
                            <>
                              <div
                                css={css`
                                  ${bonusBackgroundText};
                                  margin: 5px 0;
                                `}
                              >
                                {index === 0 && `${name} (${xpWeighting} XP)`}
                              </div>
                              <span
                                css={css`
                                  font-size: 12px;
                                  position: relative;
                                `}
                              >
                                {resourceString}
                              </span>
                            </>
                          )
                        })
                      }
                      <p
                        css={css`
                          font-size: 14px;
                          margin-top: 20px;
                        `}
                      >
                        {`Total Resource Load: ${multiplier}`}
                      </p>
                    </>
                  )
                }
              </div>
            </div>
            {
              goal ? (
                <>
                  { isObjectiveCompleted ? (
                      <div css={css`margin-top: 10px`}>{`Objective completed ${useDateFormat(parseInt(matchedCompletedObjectives[0].completed_timestamp))}`}</div>
                    ) : (
                      <div>
                        <p
                          css={css`
                            display: flex;
                            flex-direction: column;
                            margin-top: 15px;
                          `}
                        >
                          {isExpired ? `late penalty applied (+${latePenalty})` : `(${countdownView} until cost increase)`}
                        </p>
                        <p
                          css={css`
                            display: flex;
                            flex-direction: column;
                            margin-top: 15px;
                          `}
                        >
                          {`Complete Objective #${confirmedObjectiveId !== undefined ? parseInt(confirmedObjectiveId) + 1 : -1}. Your resources will be consumed and you will receive ${xpReward} XP.`}
                        </p>
                      </div>
                    )}
                </>
              )
             : (
              <></>
            )
          }
          </div>
          <Ul>
            {gasDollars !== undefined && (
              <Li>{`Transaction fee: ${(
                parseFloat(gasDollars) + 0.05
              ).toPrecision(2)} $UST`}</Li>
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
          <Button
            disabled={
              mutation.isLoading ||
              gasDollars === undefined ||
              gasFee.isFetching ||
              isObjectiveCompleted ||
              notEnoughResources !== false
            }
            sizevariant={'large'}
            colorvariant={'secondary'}
            css={css`
              margin-top: 15px;
            `}
            onClick={submit}
          >
            {'COMPLETE OBJECTIVE'}
          </Button>
          </Container>
        </>
      }
    </ModalContent>
  )
}

export default ObjectiveDetail
