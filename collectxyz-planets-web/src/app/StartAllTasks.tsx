import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import queryString from 'query-string'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import { ModalBack, ModalContent } from 'src/components/Modal'
import P from 'src/components/P'
import Select from 'src/components/Select'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { useEnvironment } from 'src/contexts/environment.context'
import useBonusTokenBalance from 'src/hooks/useBonusTokenBalance'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePlanets from 'src/hooks/usePlanets'
import usePlanetTasks from 'src/hooks/usePlanetTasks'
import usePollTxHash from 'src/hooks/usePollTxHash'
import useResourceGatheringTasks from 'src/hooks/useResourceGatheringTasks'
import useXyzTokens from 'src/hooks/useXyzTokens'
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

const StartAllTasks: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const {
    sortedTokens,
  } = useXyzTokens()

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['xyzs'])
      queryClient.invalidateQueries(['planetTasks'])
      queryClient.invalidateQueries(['resourceGatheringTasks'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const params = queryString.parse(location.search)
  const taskTypeParam = params.taskType
  const [taskType, setTaskType] = useState<string | undefined>(taskTypeParam as string | undefined)

  const ids = sortedTokens?.map((tok) => tok.name) || []

  const planets = usePlanets()
  const hasThreePlanets = planets.result.data
    ? ids.filter((id) => planets.result.data!![id]?.length === 3)
    : []
  const hasZeroPlanets = planets.result.data
    ? ids?.filter((id) => planets.result.data!![id] === undefined || planets.result.data!![id].length === 0)
    : []

  const planetTasks = usePlanetTasks()
  const resourceGatheringTasks = useResourceGatheringTasks()
  const currentTaskIds = taskType === TaskTypes.Exploration
    ? new Set([...planetTasks.result.data?.map((task) => task.nft_token_id) || [], ...hasThreePlanets])
    : new Set([...resourceGatheringTasks.result.data?.map((task) => task.nft_token_id) || [], ...hasZeroPlanets])

  const filteredIds = ids.filter((id) => (
    !currentTaskIds.has(id)
  ))
  const startTaskExecute = connectedWallet !== undefined && filteredIds !== undefined ? filteredIds.map((id) => new MsgExecuteContract(
    connectedWallet.walletAddress,
    taskType === TaskTypes.Exploration
      ? environmentContext.PLANETS_CONTRACT_ADDRESS
      : environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS,
    {
      start_task: {
        xyz_nft_id: `${id}`,
        bonus_token_count: 0,
      },
    },
    { 'uusd': 50000 },
  )) : []
  const executes = startTaskExecute
  const { result: gasFee, gasDollars } = useGasEstimate(executes)
  const tryStartAllTasks = async (): Promise<TxResult> => {
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
  unknown
  >(tryStartAllTasks, {
    onSuccess: async (data) => {
      if (data?.success) {
        setTxHash(data?.result.txhash)
      } else {
        window.alert('Transaction broadcast failed')
      }
    },
  })

  const taskTypeOptions = [
    {
      value: '',
      display: '<select type>',
    },
    {
      value: TaskTypes.Exploration,
      display: 'Exploration',
    },
    {
      value: TaskTypes.ResourceGathering,
      display: 'Resource Gathering',
    },
  ]

  const {
    result: { data: bonusTokenBalance },
  } = useBonusTokenBalance()

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
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
              <p>
                {'Tasks started! '}
              </p>
            </>
          }
        />
      )}
      {!txHash && (
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
              {'(BETA) START ALL TASKS'}
            </H2>
          </div>
          <>
            <P>
              {
                'Select the type of task to start. Every task has a fixed, real-time duration - once elapsed, return to the tasks page to complete the task.'
              }
            </P>
            <div css={css`
              display: flex;
              flex-direction: column;
            `}>
              {'Starting task for: '}
              {filteredIds?.map((id) => (<p key={id}>
                {id}
              </p>))}
            </div>
            <Select
              label={'Type'}
              value={taskType}
              options={taskTypeOptions}
              onChange={setTaskType}
            ></Select>
          </>
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
              {'START ALL TASKS'}
            </Button>
          )}
        </Form>
      )}
    </ModalContent>
  )
}

export default StartAllTasks
