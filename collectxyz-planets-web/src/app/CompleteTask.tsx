import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import queryString from 'query-string'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import { Link } from 'react-router-dom'
import { TaskTypes } from 'src/app/StartTask'
import BonusTokenCount from 'src/components/BonusTokenCount'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalBack, ModalContent, ModalTypes } from 'src/components/Modal'
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import XyzCard from 'src/components/XyzCard'
import { useEnvironment } from 'src/contexts/environment.context'
import useBonusTokenBalance, {
  BonusTokenBalance,
} from 'src/hooks/useBonusTokenBalance'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePollTxHash from 'src/hooks/usePollTxHash'
import useResourceBalance, {
  ResourceBalance,
} from 'src/hooks/useResourceBalance'
import useXyzNftInfo from 'src/hooks/useXyzNftInfo'
import useXyzPlanets from 'src/hooks/useXyzPlanets'
import useXyzPlanetTask from 'src/hooks/useXyzPlanetTask'
import useXyzResourceGatheringTask from 'src/hooks/useXyzResourceGatheringTask'
import { PlanetModel } from 'src/models/planet.models'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'

const Form = styled.form`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 330px;
`

const CompleteTask: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber
  const name = `xyz #${nameNumber}`
  const taskType = params.taskType

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

  const executes =
    connectedWallet !== undefined
      ? [
        new MsgExecuteContract(
          connectedWallet.walletAddress,
          taskType === TaskTypes.Exploration
            ? environmentContext.PLANETS_CONTRACT_ADDRESS
            : environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS,
          {
            complete_task: {
              xyz_nft_id: name,
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

  const { data } = useXyzNftInfo(name)
  const { result: planetTaskResult } = useXyzPlanetTask(name)
  const { result: resourceGatheringTaskResult } =
    useXyzResourceGatheringTask(name)
  const [cachedPlanets, setCachedPlanets] = useState<
  PlanetModel[] | undefined
  >()
  const [fetchedTransactionPlanets, setFetchedTransactionPlanets] = useState<
  PlanetModel[] | undefined
  >()
  const onSuccess = (data: PlanetModel[]) => {
    if (tx !== undefined) {
      setFetchedTransactionPlanets(data)
    }
  }
  const { result: planetsResult } = useXyzPlanets(
    data?.extension.coordinates,
    onSuccess,
  )
  if (planetsResult.data !== undefined && cachedPlanets === undefined) {
    setCachedPlanets(planetsResult.data)
  }

  const [cachedBonusTokenBalance, setCachedBonusTokenBalance] = useState<
  BonusTokenBalance | undefined
  >()
  const [bonusTokenBalance, setBonusTokenBalance] = useState<
  BonusTokenBalance | undefined
  >()
  const onBonusSuccess = (data: BonusTokenBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setBonusTokenBalance(data)
    }
  }
  const { result: bonusResult } = useBonusTokenBalance(onBonusSuccess)
  if (bonusResult.data !== undefined && cachedBonusTokenBalance === undefined) {
    setCachedBonusTokenBalance(bonusResult.data)
  }

  const [cachedXpBalance, setCachedXpBalance] = useState<
  ResourceBalance | undefined
  >()
  const [xpBalance, setXpBalance] = useState<ResourceBalance | undefined>()
  const onXpBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setXpBalance(data)
    }
  }
  const {
    result: { data: xyzXpBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_XP_CONTRACT_ADDRESS,
    onXpBalanceSuccess,
  )
  if (xyzXpBalanceData !== undefined && cachedXpBalance === undefined) {
    setCachedXpBalance(xyzXpBalanceData)
  }

  const [cachedRockBalance, setCachedRockBalance] = useState<
  ResourceBalance | undefined
  >()
  const [rockBalance, setRockBalance] = useState<ResourceBalance | undefined>()
  const onRockBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setRockBalance(data)
    }
  }
  const {
    result: { data: xyzRockBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
    onRockBalanceSuccess,
  )
  if (xyzRockBalanceData !== undefined && cachedRockBalance === undefined) {
    setCachedRockBalance(xyzRockBalanceData)
  }
  const [cachedMetalBalance, setCachedMetalBalance] = useState<
  ResourceBalance | undefined
  >()
  const [metalBalance, setMetalBalance] = useState<
  ResourceBalance | undefined
  >()
  const onMetalBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setMetalBalance(data)
    }
  }
  const {
    result: { data: xyzMetalBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
    onMetalBalanceSuccess,
  )
  if (xyzMetalBalanceData !== undefined && cachedMetalBalance === undefined) {
    setCachedMetalBalance(xyzMetalBalanceData)
  }
  const [cachedIceBalance, setCachedIceBalance] = useState<
  ResourceBalance | undefined
  >()
  const [iceBalance, setIceBalance] = useState<ResourceBalance | undefined>()
  const onIceBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setIceBalance(data)
    }
  }
  const {
    result: { data: xyzIceBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
    onIceBalanceSuccess,
  )
  if (xyzIceBalanceData !== undefined && cachedIceBalance === undefined) {
    setCachedIceBalance(xyzIceBalanceData)
  }
  const [cachedGasBalance, setCachedGasBalance] = useState<
  ResourceBalance | undefined
  >()
  const [gasBalance, setGasBalance] = useState<ResourceBalance | undefined>()
  const onGasBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setGasBalance(data)
    }
  }
  const {
    result: { data: xyzGasBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_GAS_CONTRACT_ADDRESS,
    onGasBalanceSuccess,
  )
  if (xyzGasBalanceData !== undefined && cachedGasBalance === undefined) {
    setCachedGasBalance(xyzGasBalanceData)
  }
  const [cachedWaterBalance, setCachedWaterBalance] = useState<
  ResourceBalance | undefined
  >()
  const [waterBalance, setWaterBalance] = useState<
  ResourceBalance | undefined
  >()
  const onWaterBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setWaterBalance(data)
    }
  }
  const {
    result: { data: xyzWaterBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_WATER_CONTRACT_ADDRESS,
    onWaterBalanceSuccess,
  )
  if (xyzWaterBalanceData !== undefined && cachedWaterBalance === undefined) {
    setCachedWaterBalance(xyzWaterBalanceData)
  }
  const [cachedGemBalance, setCachedGemBalance] = useState<
  ResourceBalance | undefined
  >()
  const [gemBalance, setGemBalance] = useState<ResourceBalance | undefined>()
  const onGemBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setGemBalance(data)
    }
  }
  const {
    result: { data: xyzGemBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_GEM_CONTRACT_ADDRESS,
    onGemBalanceSuccess,
  )
  if (xyzGemBalanceData !== undefined && cachedGemBalance === undefined) {
    setCachedGemBalance(xyzGemBalanceData)
  }
  const [cachedLifeBalance, setCachedLifeBalance] = useState<
  ResourceBalance | undefined
  >()
  const [lifeBalance, setLifeBalance] = useState<ResourceBalance | undefined>()
  const onLifeBalanceSuccess = (data: ResourceBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setLifeBalance(data)
    }
  }
  const {
    result: { data: xyzLifeBalanceData },
  } = useResourceBalance(
    name,
    environmentContext.XYZ_LIFE_CONTRACT_ADDRESS,
    onLifeBalanceSuccess,
  )
  if (xyzLifeBalanceData !== undefined && cachedLifeBalance === undefined) {
    setCachedLifeBalance(xyzLifeBalanceData)
  }
  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    mutation.mutate({})
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
            taskType === TaskTypes.Exploration ? (
              <>
                {fetchedTransactionPlanets === undefined && (
                  <LoadingIndicator></LoadingIndicator>
                )}
                {fetchedTransactionPlanets !== undefined &&
                  cachedPlanets !== undefined &&
                  fetchedTransactionPlanets.length <= cachedPlanets.length && (
                  <>
                    <p>{'No planets discovered.'}</p>
                    <p>
                      {'xyz rewards persistence - '}
                      <Link
                        to={`?modal=${ModalTypes.StartTask}&nameNumber=${
                          name.split('#')[1]
                        }&taskType=${TaskTypes.Exploration}`}
                        css={css`
                            color: white;
                            text-decoration: underline;
                          `}
                      >
                        {'try again?'}
                      </Link>
                    </p>
                  </>
                )}
                {fetchedTransactionPlanets !== undefined &&
                  cachedPlanets !== undefined &&
                  fetchedTransactionPlanets.length > cachedPlanets.length && (
                  <>
                    {data !== undefined && (
                      <XyzCard xyzResponse={data}></XyzCard>
                    )}
                    <p>
                      {'New planet discovered! '}
                      {fetchedTransactionPlanets.length < 3 && (
                        <Link
                          to={`?modal=${ModalTypes.StartTask}&nameNumber=${
                            name.split('#')[1]
                          }&taskType=${TaskTypes.Exploration}`}
                          css={css`
                              color: white;
                              text-decoration: underline;
                            `}
                        >
                          {'Search for more?'}
                        </Link>
                      )}
                    </p>
                  </>
                )}
                <div
                  css={css`
                    width: 150px;
                    height: 1px;
                    background-color: white;
                  `}
                />
                {xpBalance !== undefined &&
                  cachedXpBalance !== undefined &&
                  parseInt(xpBalance.balance) >
                    parseInt(cachedXpBalance.balance) && (
                  <p>{`XP earned: ${
                    (parseInt(xpBalance.balance) -
                        parseInt(cachedXpBalance.balance)) /
                      1000000
                  }`}</p>
                )}
              </>
            ) : (
              <>
                {rockBalance !== undefined &&
                  cachedRockBalance !== undefined &&
                  parseInt(rockBalance.balance) >
                    parseInt(cachedRockBalance.balance) && (
                  <p>{`ROCK gathered: ${
                    (parseInt(rockBalance.balance) -
                        parseInt(cachedRockBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                {metalBalance !== undefined &&
                  cachedMetalBalance !== undefined &&
                  parseInt(metalBalance.balance) >
                    parseInt(cachedMetalBalance.balance) && (
                  <p>{`METAL gathered: ${
                    (parseInt(metalBalance.balance) -
                        parseInt(cachedMetalBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                {iceBalance !== undefined &&
                  cachedIceBalance !== undefined &&
                  parseInt(iceBalance.balance) >
                    parseInt(cachedIceBalance.balance) && (
                  <p>{`ICE gathered: ${
                    (parseInt(iceBalance.balance) -
                        parseInt(cachedIceBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                {gasBalance !== undefined &&
                  cachedGasBalance !== undefined &&
                  parseInt(gasBalance.balance) >
                    parseInt(cachedGasBalance.balance) && (
                  <p>{`GAS gathered: ${
                    (parseInt(gasBalance.balance) -
                        parseInt(cachedGasBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                {waterBalance !== undefined &&
                  cachedWaterBalance !== undefined &&
                  parseInt(waterBalance.balance) >
                    parseInt(cachedWaterBalance.balance) && (
                  <p>{`WATER gathered: ${
                    (parseInt(waterBalance.balance) -
                        parseInt(cachedWaterBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                {gemBalance !== undefined &&
                  cachedGemBalance !== undefined &&
                  parseInt(gemBalance.balance) >
                    parseInt(cachedGemBalance.balance) && (
                  <p>{`GEM gathered: ${
                    (parseInt(gemBalance.balance) -
                        parseInt(cachedGemBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                {lifeBalance !== undefined &&
                  cachedLifeBalance !== undefined &&
                  parseInt(lifeBalance.balance) >
                    parseInt(cachedLifeBalance.balance) && (
                  <p>{`LIFE gathered: ${
                    (parseInt(lifeBalance.balance) -
                        parseInt(cachedLifeBalance.balance)) /
                      1000000
                  }`}</p>
                )}
                <Link
                  to={`?modal=${ModalTypes.StartTask}&nameNumber=${
                    name.split('#')[1]
                  }&taskType=${TaskTypes.ResourceGathering}`}
                  css={css`
                    color: white;
                    text-decoration: underline;
                  `}
                >
                  {'Gather more?'}
                </Link>
                <div
                  css={css`
                    width: 150px;
                    height: 1px;
                    background-color: white;
                  `}
                />
                {bonusTokenBalance !== undefined &&
                  cachedBonusTokenBalance !== undefined &&
                  parseInt(bonusTokenBalance.balance) >
                    parseInt(cachedBonusTokenBalance.balance) && (
                  <div
                    css={css`
                        display: flex;
                        align-items: center;
                        grid-gap: 15px;
                      `}
                  >
                    <BonusTokenCount
                      bonusTokenBalance={bonusTokenBalance}
                    ></BonusTokenCount>
                    <p>{'BONUS token found!'}</p>
                  </div>
                )}
                {xpBalance !== undefined &&
                  cachedXpBalance !== undefined &&
                  parseInt(xpBalance.balance) >
                    parseInt(cachedXpBalance.balance) && (
                  <p>{`XP earned: ${
                    (parseInt(xpBalance.balance) -
                        parseInt(cachedXpBalance.balance)) /
                      1000000
                  }`}</p>
                )}
              </>
            )
          }
        />
      )}
      {!txHash &&
        ((taskType === TaskTypes.Exploration &&
          planetTaskResult.data !== undefined) ||
          (taskType === TaskTypes.ResourceGathering &&
            resourceGatheringTaskResult.data !== undefined)) && (
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
              {'COMPLETE TASK'}
            </H2>
          </div>
          {taskType === TaskTypes.Exploration && (
            <P>
              {
                'Complete Exploration task. You have a chance to discover a planet with random resources.'
              }
            </P>
          )}
          {taskType === TaskTypes.ResourceGathering && (
            <P>
              {
                'Complete Resource Gathering task. You will acquire resources for your xyz.'
              }
            </P>
          )}
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
          {taskType && (
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
              {'COMPLETE TASK'}
            </Button>
          )}
        </Form>
      )}
    </ModalContent>
  )
}

export default CompleteTask
