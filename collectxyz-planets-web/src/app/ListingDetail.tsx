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
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { useEnvironment } from 'src/contexts/environment.context'
import useCountdown from 'src/hooks/useCountdown'
import useGasEstimate from 'src/hooks/useGasEstimate'
import useListingInfo from 'src/hooks/useListingInfo'
import usePollTxHash from 'src/hooks/usePollTxHash'
import useResourceBalance from 'src/hooks/useResourceBalance'
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

const ListingDetail: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber
  const name = `xyz #${nameNumber}`
  const { data } = useXyzNftInfo(name)
  const listingId = params.listingId
  const { data: listingData } = useListingInfo(parseInt(listingId as string))
  const isLister = name === `${listingData?.lister_xyz_id}`

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['marketplaceResourceAllowance'])
      queryClient.invalidateQueries(['resourceBalance'])
      queryClient.invalidateQueries(['listings'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const {
    amountNumber: rockAmountNumber,
  } = useResourceBalance(name, environmentContext.XYZ_ROCK_CONTRACT_ADDRESS)
  const {
    amountNumber: metalAmountNumber,
  } = useResourceBalance(name, environmentContext.XYZ_METAL_CONTRACT_ADDRESS)
  const {
    amountNumber: iceAmountNumber,
  } = useResourceBalance(name, environmentContext.XYZ_ICE_CONTRACT_ADDRESS)

  const [rmiResource, setRmiResource] = useState<string>('xyzROCK')
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
  const resources = [
    {
      id: 'xyzROCK',
      allowance: parseInt(
        xyzRockMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_ROCK_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzMETAL',
      allowance: parseInt(
        xyzMetalMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_METAL_CONTRACT_ADDRESS,
    },
    {
      id: 'xyzICE',
      allowance: parseInt(
        xyzIceMarketplaceAllowanceResult.data?.allowance || '0',
      ),
      address: environmentContext.XYZ_ICE_CONTRACT_ADDRESS,
    },
  ]

  const allowanceExecutes = connectedWallet !== undefined ? resources
    .filter(
      (resource) =>
        resource.allowance <
            parseInt(listingData?.price_rmi || '0') * 1000000 &&
          resource.id === rmiResource,
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
    ) : []
  const takeExecute = connectedWallet !== undefined ? isLister ? [new MsgExecuteContract(
    connectedWallet.walletAddress,
    environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
    {
      revoke_listing: {
        listing_id: parseInt(listingId as string),
      },
    },
  )] : [new MsgExecuteContract(
    connectedWallet.walletAddress,
    environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
    {
      take_listing: {
        taker_xyz_id: name,
        listing_id: parseInt(listingId as string),
        rmi_denom: rmiResource,
      },
    },
  )] : []
  const executes = [...allowanceExecutes, ...takeExecute]
  const { result: gasFee, gasDollars } = useGasEstimate(executes)
  const action = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    if (gasFee.data === undefined) {
      throw new Error(`Transaction fee could not be estimated. Please verify you have enough of the selected resource.`)
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
  >(action, {
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

  const { isComplete: isExpired, countdownView } = useCountdown(
    0,
    parseInt(listingData?.expired_at || '0') / 1000000,
  )

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
              <p>{isLister ? 'Listing cancelled' : 'Listing purchased!'}</p>
            </>
          }
        />
      )}
      {!txHash && data !== undefined && listingData !== undefined && (
        <Form onSubmit={onSubmit}>
          <H2
            css={css`
              align-self: center;
              text-align: center;
              ${completeTaskBackgroundText};
            `}
          >
            {`LISTING #${listingData.listing_id}`}
          </H2>
          {/* <P css={css``}>
            {
              'List a bundle of resources on the marketplace. Listings expire after 48 hours, and you can cancel a listing at any time and receive the bundle back.'
            }
          </P> */}
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 5px;
            `}
          >
            <p>{'Resources:'}</p>
            {listingData?.resources.map((resource) => (
              <p
                key={resource.id}
                css={css`
                  color: lightgray;
                  font-size: 12px;
                `}
              >
                {`${resource.id.slice(3)} - ${
                  parseInt(resource.amount) / 1000000
                }`}
              </p>
            ))}
          </div>
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 5px;
            `}
          >
            <p>{`Remaining:`}</p>
            <p
              css={css`
                color: ${isExpired ? 'darkgray' : 'lightgray'};
                font-size: 12px;
              `}
            >
              {isExpired ? 'Expired' : countdownView}
            </p>
          </div>
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 5px;
            `}
          >
            <p>{'Price'}</p>
            <p
              css={css`
                color: lightgray;
                font-size: 12px;
              `}
            >
              {`${parseInt(listingData.price_rmi) / 1000000} RMI`}
            </p>
          </div>
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 5px;
            `}
          >
            <p>{`Listed by:`}</p>
            <p
              css={css`
                color: lightgray;
                font-size: 12px;
              `}
            >
              {listingData.lister_xyz_id}
            </p>
          </div>

          {!isLister && (
            <div
              css={css`
                display: flex;
                flex-direction: column;
                grid-gap: 5px;
              `}
            >
              <p>{'Pay with'}</p>
              <div
                css={css`
                  position: relative;
                  display: flex;
                  flex-wrap: wrap;
                  grid-gap: 10px;
                `}
              >
                <FilterPill
                  selected={rmiResource === 'xyzROCK'}
                  type={'button'}
                  onClick={() => {
                    setRmiResource('xyzROCK')
                  }}
                >
                  {`${parseInt(listingData.price_rmi) / 1000000} ROCK (${rockAmountNumber})`}
                </FilterPill>
                <FilterPill
                  selected={rmiResource === 'xyzMETAL'}
                  type={'button'}
                  onClick={() => {
                    setRmiResource('xyzMETAL')
                  }}
                >
                  {`${parseInt(listingData.price_rmi) / 1000000} METAL (${metalAmountNumber})`}
                </FilterPill>
                <FilterPill
                  selected={rmiResource === 'xyzICE'}
                  type={'button'}
                  onClick={() => {
                    setRmiResource('xyzICE')
                  }}
                >
                  {`${parseInt(listingData.price_rmi) / 1000000} ICE (${iceAmountNumber})`}
                </FilterPill>
              </div>
            </div>
          )}
          <Ul>
            {gasDollars !== undefined && (
              <Li>{`Transaction fee: ${gasDollars} $UST`}</Li>
            )}
            {gasDollars === undefined && !gasFee.isFetching && (
              <Li css={css`color: lightcoral;`}>{`Transaction fee could not be estimated. Please verify you have enough of the selected resource.`}</Li>
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
          {isLister && (
            <Button
              disabled={mutation.isLoading || gasDollars === undefined || gasFee.isFetching}
              sizevariant={'large'}
              colorvariant={'secondary'}
              css={css`
                margin-top: 15px;
              `}
            >
              {'CANCEL LISTING'}
            </Button>
          )}
          {!isLister && !isExpired && (
            <Button
              disabled={mutation.isLoading || gasDollars === undefined || gasFee.isFetching}
              sizevariant={'large'}
              colorvariant={'secondary'}
              css={css`
                margin-top: 15px;
              `}
            >
              {'PURCHASE LISTING'}
            </Button>
          )}
        </Form>
      )}
    </ModalContent>
  )
}

export default ListingDetail
