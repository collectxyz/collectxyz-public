import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import { ModalBack, ModalContent } from 'src/components/Modal'
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { useCurrentTimeDate } from 'src/contexts/currentTimeDate.context'
import { useEnvironment } from 'src/contexts/environment.context'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePlanetTasks from 'src/hooks/usePlanetTasks'
import usePollTxHash from 'src/hooks/usePollTxHash'
import useResourceGatheringTasks from 'src/hooks/useResourceGatheringTasks'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'

const Form = styled.form`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 330px;
`

const CompleteAllTasks: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['xyzs'])
      queryClient.invalidateQueries(['planetTasks'])
      queryClient.invalidateQueries(['planets'])
      queryClient.invalidateQueries(['resourceGatheringTasks'])
      queryClient.invalidateQueries(['resourceBalance'])
      queryClient.invalidateQueries(['bonusTokenBalance'])
      queryClient.invalidateQueries(['latestRand'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const {currentTimeDate } = useCurrentTimeDate()

  const planetTasks = usePlanetTasks()
  const completePlanetTaskXyzIds = planetTasks.result.data?.filter((task) => {
    const isComplete = Math.floor(parseInt(task.completes) / 1000000) - currentTimeDate.getTime() <= 0
    return isComplete
  }).map((task) => task.nft_token_id)
  const resourceGatheringTasks = useResourceGatheringTasks()
  const completeResourceGatheringTaskXyzIds = resourceGatheringTasks.result.data?.filter((task) => {
    const isComplete = Math.floor(parseInt(task.completes) / 1000000) - currentTimeDate.getTime() <= 0
    return isComplete
  }).map((task) => task.nft_token_id)

  const executes =
    connectedWallet !== undefined
      ? [
        ...completePlanetTaskXyzIds?.map((id) => new MsgExecuteContract(
          connectedWallet.walletAddress,
          environmentContext.PLANETS_CONTRACT_ADDRESS,
          {
            complete_task: {
              xyz_nft_id: id,
            },
          },
        )) || [],
        ...completeResourceGatheringTaskXyzIds?.map((id) => new MsgExecuteContract(
          connectedWallet.walletAddress,
          environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS,
          {
            complete_task: {
              xyz_nft_id: id,
            },
          },
        )) || [],

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
              {'Tasks completed!'}
            </>
          }
        />
      )}
      {!txHash &&
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
              {'(BETA) COMPLETE ALL TASKS'}
            </H2>
          </div>
          <div css={css`
              display: flex;
              flex-direction: column;
            `}>
            {'Completing exploration task for: '}
            {completePlanetTaskXyzIds?.map((id) => (<p key={id}>
              {id}
            </p>))}
          </div>
          <div css={css`
              display: flex;
              flex-direction: column;
            `}>
            {'Completing resource gathering task for: '}
            {completeResourceGatheringTaskXyzIds?.map((id) => (<p key={id}>
              {id}
            </p>))}
          </div>

          <Ul>
            {gasDollars !== undefined && (
              <Li>{`Transaction fee: ${gasDollars} $UST`}</Li>
            )}
            {gasDollars === undefined && !gasFee.isFetching && (
              <Li
                css={css`
                    color: lightcoral;
                  `}
              >{`Transaction fee could not be estimated. Please try again later.`}</Li>
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
                  gasFee.isFetching
              }
              sizevariant={'large'}
              colorvariant={'secondary'}
              css={css`
                  margin-top: 15px;
                `}
            >
              {'COMPLETE TASKS'}
            </Button>
          )}
        </Form>
      }
    </ModalContent>
  )
}

export default CompleteAllTasks
