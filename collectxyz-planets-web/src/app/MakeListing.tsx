import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import queryString from 'query-string'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import Button from 'src/components/Button'
import FilterPill from 'src/components/FilterPill'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import { ModalBack, ModalContent } from 'src/components/Modal'
import NumberInput from 'src/components/NumberInput'
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { useEnvironment } from 'src/contexts/environment.context'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePollTxHash from 'src/hooks/usePollTxHash'
import useResourceBalance from 'src/hooks/useResourceBalance'
import useXyzConfig from 'src/hooks/useXyzConfig'
import useXyzMarketplaceResourceAllowance from 'src/hooks/useXyzMarketplaceResourceAllowance'
import useXyzNftInfo from 'src/hooks/useXyzNftInfo'
import { mediaDown } from 'src/styles/breakpoints'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'

const Form = styled.form`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 360px;
  ${mediaDown('md')`
    width: 100%;
  `};
`

const MakeListing: React.FC = () => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const xyzConfig = useXyzConfig()
  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber
  const name = `xyz #${nameNumber}`
  const { data } = useXyzNftInfo(name)

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)

  const [price, setPrice] = useState<number>(1000)

  const [rock, setRock] = useState<number | undefined>(0)
  const [metal, setMetal] = useState<number | undefined>(0)
  const [ice, setIce] = useState<number | undefined>(0)
  const [gas, setGas] = useState<number | undefined>(0)
  const [water, setWater] = useState<number | undefined>(0)
  const [gem, setGem] = useState<number | undefined>(0)
  const [life, setLife] = useState<number | undefined>(0)

  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['marketplaceResourceAllowance'])
      queryClient.invalidateQueries(['resourceBalance'])
      queryClient.invalidateQueries(['listings'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

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

  const { result: xyzRockMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
    )
  const { result: xyzMetalMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
    )
  const { result: xyzIceMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
    )
  const { result: xyzGasMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_GAS_CONTRACT_ADDRESS,
    )
  const { result: xyzWaterMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_WATER_CONTRACT_ADDRESS,
    )
  const { result: xyzGemMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_GEM_CONTRACT_ADDRESS,
    )
  const { result: xyzLifeMarketplaceAllowanceResult } =
    useXyzMarketplaceResourceAllowance(
      name,
      environmentContext.XYZ_LIFE_CONTRACT_ADDRESS,
    )

  const resources = [
    {
      id: 'xyzROCK',
      amount: rock,
      allowance: parseInt(
        xyzRockMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzMETAL',
      amount: metal,
      allowance: parseInt(
        xyzMetalMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzICE',
      amount: ice,
      allowance: parseInt(
        xyzIceMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzGAS',
      amount: gas,
      allowance: parseInt(
        xyzGasMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_GAS_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzWATER',
      amount: water,
      allowance: parseInt(
        xyzWaterMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_WATER_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzGEM',
      amount: gem,
      allowance: parseInt(
        xyzGemMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_GEM_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzLIFE',
      amount: life,
      allowance: parseInt(
        xyzLifeMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_LIFE_CONTRACT_ADDRESS,
    },
  ]

  const allowanceExecutes =
    connectedWallet !== undefined
      ? resources
        .filter(
          (resource) => resource.allowance < (resource.amount || 0) * 1000000,
        )
        .map(
          (resource) =>
            new MsgExecuteContract(
              connectedWallet.walletAddress,
              resource.address,
              {
                increase_allowance: {
                  owner_xyz_id: name,
                  spender: environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
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
          environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
          {
            make_listing: {
              lister_xyz_id: name,
              price_rmi: `${price * 1000000}`,
              deposit_rmi_denom: 'xyzICE',
              resources: resources
                .filter(
                  (item) => item.amount !== undefined && item.amount > 0,
                )
                .map((item) => ({
                  id: item.id,
                  amount: `${(item.amount || 0) * 1000000}`,
                })),
            },
          },
          { uusd: 50000 },
        ),
      ]
      : []
  const executes = [...allowanceExecutes, ...execute]
  const { result: gasFee, gasDollars } = useGasEstimate(executes)
  const makeListingMutation = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    if (gasFee.data === undefined) {
      throw new Error(
        `Transaction fee could not be estimated. Please verify you have enough resources, and have at least one resource selected.`,
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
  unknown
  >(makeListingMutation, {
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
              <p>
                {'Listing created! '}
                {/* <Link
                  to={`?modal=${ModalTypes.MakeListing}&nameNumber=${nameNumber}`}
                  css={css`
                    color: white;
                    text-decoration: underline;
                  `}
                >
                  {'Create another?'}
                </Link> */}
              </p>
            </>
          }
        />
      )}
      {!txHash && data !== undefined && (
        <Form onSubmit={onSubmit}>
          <H2
            css={css`
              align-self: center;
              text-align: center;
              ${completeTaskBackgroundText};
            `}
          >
            {'CREATE LISTING'}
          </H2>
          <P css={css``}>
            {
              'List a bundle of resources on the marketplace. Listings expire after 48 hours, and you can cancel a listing at any time and receive the bundle back.'
            }
          </P>
          <P css={css``}>
            {
              'You must choose one of the prices below for the bundle. Listings are priced in RMI, which stands for ROCK, METAL, or ICE: it is up to the discretion of the purchaser to choose which resource they wish to pay in.'
            }
          </P>
          <div
            css={css`
              display: flex;
              justify-content: space-between;
              grid-gap: 10px;
            `}
          >
            <div
              css={css`
                display: flex;
                flex-direction: column;
                grid-gap: 10px;
              `}
            >
              <NumberInput
                label={`GAS (${gasAmountNumber})`}
                value={gas}
                onChange={setGas}
                placeholder={'Amount of gas to include in bundle'}
              />
              <NumberInput
                label={`WATER (${waterAmountNumber})`}
                value={water}
                onChange={setWater}
                placeholder={'Amount of water to include in bundle'}
              />
              <NumberInput
                label={`GEM (${gemAmountNumber})`}
                value={gem}
                onChange={setGem}
                placeholder={'Amount of gem to include in bundle'}
              />
              <NumberInput
                label={`LIFE (${lifeAmountNumber})`}
                value={life}
                onChange={setLife}
                placeholder={'Amount of life to include in bundle'}
              />
            </div>
            <div
              css={css`
                display: flex;
                flex-direction: column;
                grid-gap: 10px;
              `}
            >
              <NumberInput
                label={`ROCK (${rockAmountNumber})`}
                value={rock}
                onChange={setRock}
                placeholder={'Amount of rock to include in bundle'}
              />
              <NumberInput
                label={`METAL (${metalAmountNumber})`}
                value={metal}
                onChange={setMetal}
                placeholder={'Amount of metal to include in bundle'}
              />
              <NumberInput
                label={`ICE (${iceAmountNumber})`}
                value={ice}
                onChange={setIce}
                placeholder={'Amount of ice to include in bundle'}
              />
            </div>
          </div>
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 5px;
            `}
          >
            <p>{'Listing price'}</p>
            <div
              css={css`
                position: relative;
                display: flex;
                flex-wrap: wrap;
                grid-gap: 10px;
              `}
            >
              <FilterPill
                selected={price === 5}
                type={'button'}
                onClick={() => {
                  setPrice(5)
                }}
              >
                {'5 RMI'}
              </FilterPill>
              <FilterPill
                selected={price === 10}
                type={'button'}
                onClick={() => {
                  setPrice(10)
                }}
              >
                {'10 RMI'}
              </FilterPill>
              <FilterPill
                selected={price === 50}
                type={'button'}
                onClick={() => {
                  setPrice(50)
                }}
              >
                {'50 RMI'}
              </FilterPill>
              <FilterPill
                selected={price === 100}
                type={'button'}
                onClick={() => {
                  setPrice(100)
                }}
              >
                {'100 RMI'}
              </FilterPill>
              <FilterPill
                selected={price === 500}
                type={'button'}
                onClick={() => {
                  setPrice(500)
                }}
              >
                {'500 RMI'}
              </FilterPill>
              <FilterPill
                selected={price === 1000}
                type={'button'}
                onClick={() => {
                  setPrice(1000)
                }}
              >
                {'1000 RMI'}
              </FilterPill>
            </div>
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
              >{`Transaction fee could not be estimated. Please verify you have enough resources, and have at least one resource selected.`}</Li>
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
            {'CREATE LISTING'}
          </Button>
        </Form>
      )}
    </ModalContent>
  )
}

export default MakeListing
