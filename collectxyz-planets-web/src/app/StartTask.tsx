import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import queryString from 'query-string'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import BonusTokenCount from 'src/components/BonusTokenCount'
import Button from 'src/components/Button'
import ExplorationTask from 'src/components/ExplorationTask'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalBack, ModalContent } from 'src/components/Modal'
import P from 'src/components/P'
import ResourceGatheringTask from 'src/components/ResourceGatheringTask'
import Select from 'src/components/Select'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { useEnvironment } from 'src/contexts/environment.context'
import useBonusTokenBalance from 'src/hooks/useBonusTokenBalance'
import useBonusTokenPlanetsContractAllowance from 'src/hooks/useBonusTokenPlanetsContractAllowance'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePlanetsConfig from 'src/hooks/usePlanetsConfig'
import usePollTxHash from 'src/hooks/usePollTxHash'
import useXyzNftInfo from 'src/hooks/useXyzNftInfo'
import useXyzPlanets from 'src/hooks/useXyzPlanets'
import useXyzPlanetTask from 'src/hooks/useXyzPlanetTask'
import useXyzResourceGatheringTask from 'src/hooks/useXyzResourceGatheringTask'
import { TaskResponse } from 'src/models/task.models'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'

export enum TaskTypes {
  Exploration = 'exploration',
  ResourceGathering = 'resourceGathering',
}

const Form = styled.form`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 304px;
`

const StartTask: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['xyzs'])
      queryClient.invalidateQueries(['planetTasks'])
      queryClient.invalidateQueries(['resourceGatheringTasks'])
      queryClient.invalidateQueries(['bonusTokenBalance'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber
  const name = `xyz #${nameNumber}`
  const { data } = useXyzNftInfo(name)
  const taskTypeParam = params.taskType
  const [taskType, setTaskType] = useState<string | undefined>(taskTypeParam as string | undefined)
  const { result } = useBonusTokenPlanetsContractAllowance()

  const [spendBonusToken, setSpendBonusToken] = useState(false)

  const allowanceExecute = connectedWallet !== undefined ? [new MsgExecuteContract(
    connectedWallet.walletAddress,
    environmentContext.BONUS_TOKEN_CONTRACT_ADDRESS,
    {
      increase_allowance: {
        spender: environmentContext.PLANETS_CONTRACT_ADDRESS,
        amount: `${1000000 * 1000000000}`, // 1 billion allowance
        expires: {
          never: {},
        },
      },
    },
  )] : []
  const startTaskExecute = connectedWallet !== undefined ? [new MsgExecuteContract(
    connectedWallet.walletAddress,
    taskType === TaskTypes.Exploration
      ? environmentContext.PLANETS_CONTRACT_ADDRESS
      : environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS,
    {
      start_task: {
        xyz_nft_id: `xyz #${nameNumber}`,
        bonus_token_count: spendBonusToken ? 1 : 0,
      },
    },
    { 'uusd': 50000 },
  )] : []
  const executes = result.data !== undefined &&
  parseInt(result.data?.allowance) === 0 &&
  spendBonusToken
    ? [...allowanceExecute, ...startTaskExecute]
    : [...startTaskExecute]
  const { result: gasFee, gasDollars } = useGasEstimate(executes)
  const tryStartTask = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    if (gasFee.data === undefined) {
      throw new Error(`Transaction fee could not be estimated.`)
    }
    return connectedWallet.post({
      msgs: executes,
      fee: gasFee.data,
    })
  }
  const mutation = useMutation<
  TxResult | undefined,
  { message: string },
  boolean
  >(tryStartTask, {
    onSuccess: async (data) => {
      if (data?.success) {
        setTxHash(data?.result.txhash)
      } else {
        window.alert('Transaction broadcast failed')
      }
    },
  })

  const [fetchedTask, setFetchedPlanetTask] = useState<
  TaskResponse | undefined
  >()
  const onSuccess = (data: TaskResponse | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setFetchedPlanetTask(data)
    }
  }
  const {
    result: { data: planetTask, isLoading: isPlanetTaskLoading },
  } = useXyzPlanetTask(name, onSuccess)
  const {
    result: {
      data: resourceGatheringTask,
      isLoading: isResourceGatheringTaskLoading,
    },
  } = useXyzResourceGatheringTask(name, onSuccess)
  const { result: planetsResult } = useXyzPlanets(data?.extension.coordinates)
  const { data: planetsConfig } = usePlanetsConfig()
  const optionsLoading = isPlanetTaskLoading || isResourceGatheringTaskLoading
  const hasPlanetTaskOption =
    planetTask === undefined &&
    planetsResult.data !== undefined &&
    planetsConfig !== undefined &&
    planetsResult.data.length < planetsConfig.maximum_planets_per_coord
  const hasResourceGatheringTaskOption = resourceGatheringTask === undefined && planetsResult.data !== undefined && planetsResult.data.length > 0
  const hasOptions = hasPlanetTaskOption || hasResourceGatheringTaskOption
  const taskTypeOptions = [
    {
      value: '',
      display: '<select type>',
    },
    {
      value: hasPlanetTaskOption ? TaskTypes.Exploration : '',
      display: hasPlanetTaskOption
        ? 'Exploration'
        : 'Exploration (unavailable)',
    },
    {
      value: hasResourceGatheringTaskOption ? TaskTypes.ResourceGathering : '',
      display: hasResourceGatheringTaskOption
        ? 'Resource Gathering'
        : 'Resource Gathering (unavailable)',
    },
  ]

  const {
    result: { data: bonusTokenBalance },
  } = useBonusTokenBalance()

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    mutation.mutate(spendBonusToken)
  }

  return (
    <ModalContent>
      <ModalBack></ModalBack>
      <P
        css={css`
          color: lightgray;
          font-size: 16px;
          align-self: center;
          position: absolute;
          top: 5px;
          right: 5px;
        `}
      >
        {`${name}`}
      </P>
      {!!txHash && (
        <TxLoading
          tx={tx}
          txHash={txHash}
          successElement={
            <>
              {fetchedTask === undefined && (
                <LoadingIndicator></LoadingIndicator>
              )}
              {fetchedTask !== undefined && (
                <>
                  {taskType === TaskTypes.Exploration && (
                    <ExplorationTask task={fetchedTask}></ExplorationTask>
                  )}
                  {taskType === TaskTypes.ResourceGathering && (
                    <ResourceGatheringTask task={fetchedTask}></ResourceGatheringTask>
                  )}

                </>
              )}
              <p>
                {'Task started! '}
              </p>
            </>
          }
        />
      )}
      {!txHash && data !== undefined && (
        <Form onSubmit={onSubmit}>
          <div
            css={css`
              display: flex;
              flex-direction: column;
            `}
          >
            <H2
              css={css`
                align-self: center;
                ${completeTaskBackgroundText};
              `}
            >
              {'START TASK'}
            </H2>
          </div>
          {!optionsLoading && hasOptions && (
            <>
              <P>
                {
                  'Select the type of task to start. Every task has a fixed, real-time duration - once elapsed, return to the tasks page to complete the task.'
                }
              </P>
              <Select
                label={'Type'}
                value={taskType}
                options={taskTypeOptions}
                onChange={setTaskType}
              ></Select>
            </>
          )}
          {!optionsLoading && !hasOptions && (
            <P>
              {
                'No tasks available.'
              }
            </P>
          )}
          {optionsLoading && (
            <div
              css={css`
                align-self: center;
              `}
            >
              <LoadingIndicator></LoadingIndicator>
            </div>
          )}

          {taskType === TaskTypes.Exploration && (
            <>
              <Ul>
                <Li>{'Duration: 3 days'}</Li>
                <Li>{'Reward: chance to discover a planet'}</Li>
                {gasDollars !== undefined && (
                  <Li>{`Transaction fee: ${(parseFloat(gasDollars) + 0.05).toPrecision(2)} $UST`}</Li>
                )}
                {gasDollars === undefined && !gasFee.isFetching && (
                  <Li css={css`color: lightcoral;`}>{`Transaction fee could not be estimated.`}</Li>
                )}
              </Ul>
              {bonusTokenBalance !== undefined &&
                parseInt(bonusTokenBalance.balance) >= 1000000 && (
                <>
                  <div
                    css={css`
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                      `}
                  >
                    <div
                      css={css`
                          display: flex;
                          align-items: center;
                        `}
                    >
                      <input
                        type="checkbox"
                        id={'bonus'}
                        checked={spendBonusToken}
                        onChange={() => {
                          setSpendBonusToken((val) => !val)
                        }}
                        css={css`
                            margin-right: 5px;
                          `}
                      />
                      <label htmlFor="bonus">{'Spend bonus token?'}</label>
                    </div>
                    <BonusTokenCount
                      bonusTokenBalance={bonusTokenBalance}
                    ></BonusTokenCount>
                  </div>
                  <Ul>
                    <Li>
                      {
                        'Benefits: guaranteed planet discovery, with higher chance for rare resources'
                      }
                    </Li>
                  </Ul>
                </>
              )}
            </>
          )}
          {taskType === TaskTypes.ResourceGathering && (
            <>
              <Ul>
                <Li>{'Duration: 2 days'}</Li>
                <Li>{'Reward: gather resources, with a chance to find a bonus token'}</Li>
                {gasDollars !== undefined && (
                  <Li>{`Transaction fee: ${(parseFloat(gasDollars) + 0.05).toPrecision(2)} $UST`}</Li>
                )}
                {gasDollars === undefined && !gasFee.isFetching && (
                  <Li css={css`color: lightcoral;`}>{`Transaction fee could not be estimated.`}</Li>
                )}
              </Ul>
            </>
          )}

          {mutation.error && (
            <P
              css={css`
                color: red;
              `}
            >
              {mutation.error.message}
            </P>
          )}
          {taskType && (
            <Button
              disabled={mutation.isLoading || gasDollars === undefined || gasFee.isFetching}
              sizevariant={'large'}
              colorvariant={'secondary'}
              css={css`
                margin-top: 15px;
              `}
            >
              {'START TASK'}
            </Button>
          )}
        </Form>
      )}
    </ModalContent>
  )
}

export default StartTask
