import { MsgExecuteContract, StdFee } from '@terra-money/terra.js'
import { useConnectedWallet } from '@terra-money/wallet-provider'
import axios, { AxiosResponse } from 'axios'
import React, { useEffect, useState } from 'react'
import ReCAPTCHA from 'react-google-recaptcha'
import { useMutation, useQueryClient } from 'react-query'
import { Link } from 'react-router-dom'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalBack, ModalContent } from 'src/components/Modal'
import NumberInput from 'src/components/NumberInput'
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import XyzCard from 'src/components/XyzCard'
import { useEnvironment } from 'src/contexts/environment.context'
import useIsMintAvailable from 'src/hooks/useIsMintAvailable'
import usePollTxHash from 'src/hooks/usePollTxHash'
import { useTerraClient } from 'src/hooks/useTerraClient'
import useXyzConfig from 'src/hooks/useXyzConfig'
import useXyzTokens, { XyzTokensWithCoords } from 'src/hooks/useXyzTokens'
import { XyzResponse } from 'src/models/xyz.models'
import { mediaDown } from 'src/styles/breakpoints'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import { randIntRange } from 'src/utils'
import styled, { css } from 'styled-components'

const Form = styled.form`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 304px;
  ${mediaDown('md')`
    width: 100%;
  `};
`

const Mint: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const {
    xyzWalletAmount,
    xyzWalletLimit,
    xyzNumTokens,
    xyzTokenSupply,
    isMintAvailable,
    walletMaxReached,
    globalMaxReached,
    publicMintingEnabled,
    isDataLoading,
  } = useIsMintAvailable()

  const xyzConfig = useXyzConfig()

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['xyzs'])
      queryClient.invalidateQueries(['num_tokens'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const { api } = useTerraClient()
  const postCaptcha = async (body: {
    recaptchaToken: string
    data: { x: number, y: number, z: number }
  }): Promise<AxiosResponse<{ signature: string }>> => {
    if (api === undefined) {
      throw new Error('No api available')
    }

    await api!!
      .contractQuery(environmentContext.XYZ_CONTRACT_ADDRESS, {
        xyz_nft_info_by_coords: {
          coordinates: body.data,
        },
      })
      .then(
        () => {
          throw new Error('Coordinates already occupied. Please choose new coordinates.')
        },
        (error) => {
          console.log(error)
        },
      )
    return axios.post('/verify', body, {
      baseURL: environmentContext.CAPTCHA_URL,
    })
  }

  const mutation = useMutation<
  AxiosResponse<{ signature: string }>,
  { message: string },
  { recaptchaToken: string, data: { x: number, y: number, z: number } }
  >(postCaptcha, {
    onSuccess: async (data, variables) => {
      if (connectedWallet) {
        const execute = new MsgExecuteContract(
          connectedWallet.walletAddress,
          environmentContext.XYZ_CONTRACT_ADDRESS,
          {
            mint: {
              coordinates: variables.data,
              captcha_signature: data.data.signature,
            },
          },
          xyzConfig.data !== undefined
            ? { [xyzConfig.data.mint_fee.denom]: xyzConfig.data.mint_fee.amount }
            : undefined, // coins
        )
        const obj = new StdFee(1000_000, { uusd: 350000 })
        const post = await connectedWallet.post({
          msgs: [execute],
          fee: obj,
        })
        if (post.success) {
          setTxHash(post.result.txhash)
        } else {
          window.alert('Transaction broadcast failed')
        }
      }
    },
  })

  const [x, setX] = useState<number | undefined>()
  const [y, setY] = useState<number | undefined>()
  const [z, setZ] = useState<number | undefined>()
  const [captcha, setCaptcha] = useState<string | undefined>(undefined)
  const onCaptchaChange = (val: string | null) => {
    setCaptcha(val || undefined)
  }
  const [isCoordinatesAvailable, setIsCoordinatesAvailable] = useState<boolean | undefined>(undefined)
  const onBlur = () => {
    if (x !== undefined && y !== undefined && z !== undefined && api !== undefined) {
      api
        .contractQuery(environmentContext.XYZ_CONTRACT_ADDRESS, {
          xyz_nft_info_by_coords: {
            coordinates: {
              x, y, z,
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

  const randomizeCoordinates = () => {
    setX(randIntRange(-1000, 1000))
    setY(randIntRange(-1000, 1000))
    setZ(randIntRange(-1000, 1000))
  }

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (
      captcha !== undefined &&
      x !== undefined &&
      y !== undefined &&
      z !== undefined &&
      connectedWallet !== undefined
    ) {
      mutation.mutate({
        recaptchaToken: captcha,
        data: { x, y, z },
      })
    }
  }
  const [fetchedXyzResponse, setFetchedXyzResponse] = useState<XyzResponse>()
  const onSuccess = (data: XyzTokensWithCoords) => {
    const sortedTokens = data.tokens.sort(
      (a, b) => parseInt(b.name.split('#')[1]) - parseInt(a.name.split('#')[1]),
    )
    if (tx !== undefined) {
      setFetchedXyzResponse(sortedTokens[0])
    }
  }
  useXyzTokens(onSuccess)

  return (
    <ModalContent>
      <ModalBack></ModalBack>
      {xyzNumTokens.data !== undefined &&
        xyzTokenSupply !== undefined &&
        xyzWalletAmount !== undefined &&
        xyzWalletLimit !== undefined &&
        publicMintingEnabled && (
        <div
          css={css`
              position: absolute;
              top: 5px;
              right: 5px;
              text-align: right;
              color: darkGrey;
            `}
        >
          <P>{`Total minted: ${xyzNumTokens.data?.count} / ${xyzTokenSupply}`}</P>
          <P>{`Wallet minted: ${xyzWalletAmount} / ${xyzWalletLimit}`}</P>
        </div>
      )}
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
                {'xyz minted! '}
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
      {!txHash && (
        <Form onSubmit={onSubmit}>
          <H2
            css={css`
              text-align: center;
              ${completeTaskBackgroundText};
            `}
          >
            {'MINT'}
          </H2>
          {isMintAvailable && (
            <>
              <P css={css``}>
                {
                  'Enter the desired coordinates for your new xyz. All numbers must be integers between -1000 and 1000, and the chosen coordinates must not already be occupied.'
                }
              </P>

              <NumberInput label={'x'} value={x} onChange={setX} onBlur={onBlur} placeholder={`Enter x coordinate...`}/>
              <NumberInput label={'y'} value={y} onChange={setY} onBlur={onBlur} placeholder={`Enter y coordinate...`}/>
              <NumberInput label={'z'} value={z} onChange={setZ} onBlur={onBlur} placeholder={`Enter z coordinate...`}/>
              <P
                onClick={randomizeCoordinates}
                role={'button'}
                css={css`
                  text-decoration: underline;
                  cursor: pointer;
                `}
              >
                {'Randomize coordinates?'}
              </P>
              <ReCAPTCHA
                sitekey={environmentContext.CAPTCHA_PUBLIC_KEY}
                onChange={onCaptchaChange}
                css={css`
                  border-radius: 0px;
                `}
              />
              <Ul>
                <Li>{'Transaction fee: 0.35 $UST'}</Li>
                {xyzConfig.data !== undefined &&
                  parseInt(xyzConfig.data.mint_fee.amount) > 0 && (
                  <Li>{`Minting fee: ${
                    parseInt(xyzConfig.data.mint_fee.amount) / 1000000
                  } $UST`}</Li>
                )}
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
                <>
                  <P
                    css={css`
                      color: red;
                    `}
                  >
                    {mutation.error.message}
                  </P>
                </>
              )}
              <Button
                disabled={mutation.isLoading}
                sizevariant={'large'}
                colorvariant={'secondary'}
                css={css`
                  margin-top: 15px;
                `}
              >
                {'MINT'}
              </Button>
            </>
          )}
          {!isMintAvailable && (
            <>
              {isDataLoading && (
                <LoadingIndicator css={css`
                  align-self: center;
                `}/>
              )}
              {!isDataLoading && !publicMintingEnabled && (
                <P css={css`text-align:center;`}>
                  {
                    'Public minting is currently disabled.'
                  }
                </P>
              )}
              {!isDataLoading && globalMaxReached && publicMintingEnabled && (
                <P css={css`text-align:center;`}>
                  {
                    'All available xyz have been minted for this round. More minting rounds will follow!'
                  }
                </P>
              )}
              {!isDataLoading && walletMaxReached && publicMintingEnabled && (
                <P css={css`text-align:center;`}>
                  {
                    'Your wallet owns the maximum number of allowed xyz. If you wish to mint more xyz, create a new wallet or wait for an increase the per wallet limit.'
                  }
                </P>
              )}
            </>
          )}
        </Form>
      )}
    </ModalContent>
  )
}

export default Mint
