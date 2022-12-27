import { MsgExecuteContract } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import queryString from 'query-string'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import { Link } from 'react-router-dom'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalBack, ModalContent } from 'src/components/Modal'
import NumberInput from 'src/components/NumberInput'
import P from 'src/components/P'
import PlanetsWithResources from 'src/components/PlanetsWithResources'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import XyzCard from 'src/components/XyzCard'
import { useEnvironment } from 'src/contexts/environment.context'
import useGasEstimate from 'src/hooks/useGasEstimate'
import usePollTxHash from 'src/hooks/usePollTxHash'
import { useTerraClient } from 'src/hooks/useTerraClient'
import useXyzConfig from 'src/hooks/useXyzConfig'
import useXyzNftInfo from 'src/hooks/useXyzNftInfo'
import useXyzPlanets from 'src/hooks/useXyzPlanets'
import { XyzResponse } from 'src/models/xyz.models'
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

const MoveCoordinates: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const xyzConfig = useXyzConfig()

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  const params = queryString.parse(location.search)
  const nameNumber = params.nameNumber
  const name = `xyz #${nameNumber}`
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['xyzs'])
      queryClient.invalidateQueries(['planets'])
      queryClient.invalidateQueries(['planetTasks'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const { api } = useTerraClient()
  const [x, setX] = useState<number | undefined>()
  const [y, setY] = useState<number | undefined>()
  const [z, setZ] = useState<number | undefined>()
  const coordinates =
    x !== undefined && y !== undefined && z !== undefined
      ? {
        x,
        y,
        z,
      }
      : undefined

  const [isCoordinatesAvailable, setIsCoordinatesAvailable] = useState<
  boolean | undefined
  >(undefined)
  const onBlur = () => {
    if (
      x !== undefined &&
      y !== undefined &&
      z !== undefined &&
      api !== undefined
    ) {
      api
        .contractQuery(environmentContext.XYZ_CONTRACT_ADDRESS, {
          xyz_nft_info_by_coords: {
            coordinates: {
              x,
              y,
              z,
            },
          },
        })
        .then(
          () => {
            setIsCoordinatesAvailable(false)
          },
          () => {
            setIsCoordinatesAvailable(true)
          },
        )
    }
  }
  const [fetchedXyzResponse, setFetchedXyzResponse] = useState<XyzResponse>()
  const onSuccess = (data: XyzResponse | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setFetchedXyzResponse(data)
    }
  }
  const { data } = useXyzNftInfo(name, onSuccess)
  const { result: planetsResult } = useXyzPlanets(
    x !== undefined && y !== undefined && z !== undefined
      ? { x, y, z }
      : undefined,
  )

  useEffect(() => {
    if (data?.extension !== undefined) {
      setX(data.extension.coordinates.x)
      setY(data.extension.coordinates.y)
      setZ(data.extension.coordinates.z)
    }
  }, [
    data?.extension.coordinates.x,
    data?.extension.coordinates.y,
    data?.extension.coordinates.z,
  ])

  const stepsX =
    data !== undefined && x !== undefined
      ? Math.abs(data.extension.coordinates.x - x)
      : undefined
  const stepsY =
    data !== undefined && y !== undefined
      ? Math.abs(data.extension.coordinates.y - y)
      : undefined
  const stepsZ =
    data !== undefined && z !== undefined
      ? Math.abs(data.extension.coordinates.z - z)
      : undefined
  const moveFeeAmount =
    data !== undefined &&
    stepsX !== undefined &&
    stepsY !== undefined &&
    stepsZ !== undefined &&
    xyzConfig.data !== undefined
      ? parseInt(xyzConfig.data.base_move_fee.amount) +
        (stepsX + stepsY + stepsZ) * parseInt(xyzConfig.data.move_fee_per_step)
      : undefined
  const moveNanos =
    data !== undefined &&
    stepsX !== undefined &&
    stepsY !== undefined &&
    stepsZ !== undefined &&
    xyzConfig.data !== undefined
      ? xyzConfig.data.base_move_nanos + (stepsX + stepsY + stepsZ) * xyzConfig.data.move_nanos_per_step
      : undefined
  const moveMillisecondsOrZero =
    moveNanos !== undefined ? moveNanos / 1000000 : 0

  const seconds = Math.floor((moveMillisecondsOrZero / 1000) % 60)
  const minutes = Math.floor((moveMillisecondsOrZero / (1000 * 60)) % 60)
  const hours = Math.floor((moveMillisecondsOrZero / (1000 * 60 * 60)))
  const moveDurationView = `${hours.toString().padStart(2, '0')}:${minutes
    .toString()
    .padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`

  const executes =
    connectedWallet !== undefined &&
    xyzConfig.data !== undefined &&
    moveFeeAmount !== undefined
      ? [
        new MsgExecuteContract(
          connectedWallet.walletAddress,
          environmentContext.XYZ_CONTRACT_ADDRESS,
          {
            move: {
              coordinates: coordinates,
              token_id: name,
            },
          },
          { [xyzConfig.data.base_move_fee.denom]: moveFeeAmount }, // coins
        ),
      ]
      : []
  const { result: gasFee, gasDollars } = useGasEstimate(executes)
  const moveMutation = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    if (xyzConfig.data === undefined) {
      throw new Error('no xyz configuration')
    }
    if (moveFeeAmount === undefined) {
      throw new Error('no move fee provided')
    }
    if (api === undefined) {
      throw new Error('No api available')
    }
    if (coordinates === undefined) {
      throw new Error('No coordinates provided')
    }
    if (gasFee.data === undefined) {
      throw new Error(`Transaction fee could not be estimated. Please verify that the coordinates are not occupied and are valid.`)
    }
    await api
      .contractQuery(environmentContext.XYZ_CONTRACT_ADDRESS, {
        xyz_nft_info_by_coords: {
          coordinates: coordinates,
        },
      })
      .then(
        () => {
          throw new Error(
            'Coordinates already occupied. Please choose new coordinates.',
          )
        },
        (error) => {
          console.log(error)
        },
      )
    return connectedWallet.post({
      msgs: executes,
      fee: gasFee.data,
    })
  }
  const mutation = useMutation<
  TxResult | undefined,
  { message: string },
  unknown
  >(moveMutation, {
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
    if (x !== undefined && y !== undefined && z !== undefined) {
      mutation.mutate({})
    }
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
              {fetchedXyzResponse === undefined && (
                <LoadingIndicator></LoadingIndicator>
              )}
              {fetchedXyzResponse !== undefined && (
                <XyzCard xyzResponse={fetchedXyzResponse}></XyzCard>
              )}
              <p>
                {'xyz relocation in progress! '}
                <Link
                  to={'/collection'}
                  css={css`
                    color: white;
                    text-decoration: underline;
                  `}
                >
                  {'View collection'}
                </Link>
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
            {'RELOCATE'}
          </H2>
          <P css={css``}>
            {
              'Enter the desired coordinates to relocate your xyz to. Relocation costs more $UST and takes more time the further you travel.'
            }
          </P>
          <P css={css``}>
            {
              'You may relocate as many times as you want; however, you may only relocate to coordinates that are not currently occupied.'
            }
          </P>
          <P css={css``}>
            <span
              css={css`
                color: red;
              `}
            >
              {'WARNING: '}
            </span>
            {
              'relocating will permanently update the coordinates of your xyz NFT. Any data attached to your old coordinates, INCLUDING any discovered planets, will no longer be associated with your xyz; any data attached to the new coordinates will now be associated with your xyz instead. As soon as you relocate, somebody else may claim your old coordinates, along with all data attached.'
            }
          </P>
          <NumberInput
            label={'x'}
            value={x}
            onChange={setX}
            onBlur={onBlur}
            placeholder={`Enter x coordinate...`}
          />
          <NumberInput
            label={'y'}
            value={y}
            onChange={setY}
            onBlur={onBlur}
            placeholder={`Enter y coordinate...`}
          />
          <NumberInput
            label={'z'}
            value={z}
            onChange={setZ}
            onBlur={onBlur}
            placeholder={`Enter z coordinate...`}
          />
          <P>{'Planets at these coordinates:'}</P>
          {planetsResult.data !== undefined &&
            planetsResult.data.length === 0 && <P>{'No planets found'}</P>}

          {planetsResult.data !== undefined &&
            planetsResult.data.length > 0 && (
            <PlanetsWithResources
              planets={planetsResult.data}
              xyzResponse={data}
            />
          )}
          <Ul>
            {gasDollars !== undefined && (
              <Li>{`Transaction fee: ${gasDollars} $UST`}</Li>
            )}
            {gasDollars === undefined && !gasFee.isFetching && (
              <Li css={css`color: lightcoral;`}>{`Transaction fee could not be estimated. Please verify that the coordinates are not occupied and are valid.`}</Li>
            )}
            {moveFeeAmount !== undefined && (
              <Li>{`Relocation Fee: ${moveFeeAmount / 1000000} $UST`}</Li>
            )}
            {moveNanos !== undefined && (
              <Li>{`Travel Duration: ${moveDurationView}`}</Li>
            )}
            {moveFeeAmount === undefined && (
              <Li>{`Relocation Fee: unknown`}</Li>
            )}
            {moveNanos === undefined && <Li>{`Travel Duration: unknown`}</Li>}
          </Ul>
          {isCoordinatesAvailable === false && (
            <P
              css={css`
                color: red;
              `}
            >
              {'Coordinates already occupied. Please choose new coordinates.'}
            </P>
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
          <Button
            disabled={mutation.isLoading || gasDollars === undefined || gasFee.isFetching}
            sizevariant={'large'}
            colorvariant={'secondary'}
            css={css`
              margin-top: 15px;
            `}
          >
            {'RELOCATE'}
          </Button>
        </Form>
      )}
    </ModalContent>
  )
}

export default MoveCoordinates
