import { MsgExecuteContract, StdFee } from '@terra-money/terra.js'
import { TxResult, useConnectedWallet } from '@terra-money/wallet-provider'
import React, { useEffect, useState } from 'react'
import { useMutation, useQueryClient } from 'react-query'
import BonusTokenCount from 'src/components/BonusTokenCount'
import Button from 'src/components/Button'
import H2 from 'src/components/H2'
import Li from 'src/components/Li'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalBack, ModalContent } from 'src/components/Modal'
import P from 'src/components/P'
import TxLoading from 'src/components/TxLoading'
import Ul from 'src/components/Ul'
import { useEnvironment } from 'src/contexts/environment.context'
import useBonusTokenBalance, { BonusTokenBalance } from 'src/hooks/useBonusTokenBalance'
import useBonusTokenCountdown from 'src/hooks/useBonusTokenCountdown'
import usePollTxHash from 'src/hooks/usePollTxHash'
import { bonusBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'

const Form = styled.form`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 320px;
`

const ClaimBonus: React.FC = (): React.ReactElement => {
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  const [txHash, setTxHash] = useState('')
  const tx = usePollTxHash(txHash)
  useEffect(() => {
    if (tx) {
      queryClient.invalidateQueries(['bonusTokenBalance'])
      queryClient.invalidateQueries(['latestRand'])
      queryClient.invalidateQueries(['activityFeed'])
    }
  }, [tx])

  const tryClaimDiscovery = async (): Promise<TxResult> => {
    if (connectedWallet === undefined) {
      throw new Error('Could not find a connected wallet!')
    }
    const execute = new MsgExecuteContract(
      connectedWallet.walletAddress,
      environmentContext.RANDOMNESS_CONTRACT_ADDRESS,
      {
        update_rand: {},
      },
    )
    const obj = new StdFee(800_000, { uusd: 150000 })
    return connectedWallet.post({
      msgs: [execute],
      fee: obj,
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
  const [bonusTokenBalance, setBonusTokenBalance] = useState<
  BonusTokenBalance | undefined
  >()
  const onSuccess = (data: BonusTokenBalance | undefined) => {
    if (tx !== undefined && data !== undefined) {
      setBonusTokenBalance(data)
    }
  }
  useBonusTokenBalance(onSuccess)

  const { renderCountdown, countdownView} = useBonusTokenCountdown()

  return (
    <ModalContent css={css``}>
      <ModalBack></ModalBack>
      {!!txHash && (
        <TxLoading
          tx={tx}
          txHash={txHash}
          successElement={
            <>
              {bonusTokenBalance === undefined && (
                <LoadingIndicator></LoadingIndicator>
              )}
              {bonusTokenBalance !== undefined && (
                <BonusTokenCount bonusTokenBalance={bonusTokenBalance}></BonusTokenCount>
              )}
              <p>{'BONUS token claimed!'}</p>
              <P css={css`
              text-align: center;
            `}>
                {'BONUS tokens can be spent when searching for planets to guarantee a discovery and enhance the potential resources on that planet.'}
              </P>
            </>
          }
        />
      )}
      {!txHash && (
        <>
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
                  ${bonusBackgroundText};
                `}
              >
                {'CLAIM BONUS'}
              </H2>
              <p css={css`
              align-self: center;
              color: white; 
            `}>

                {`Next bonus: ${renderCountdown ? countdownView : 'Now!'}`}
              </p>
            </div>
            <P>
              {'A '}
              <span
                css={css`
                  font-weight: 1000;
                `}
              >
                {'BONUS'}
              </span>
              {' token is available when the timer hits 0! Only one person can receive the '}
              <span css={css``}>{'BONUS'}</span>
              {' token, so claim it quickly before somebody else does!'}
            </P>
            <P>
              {'BONUS tokens can be spent when searching for planets to guarantee a discovery and enhance the potential resources on that planet.'}
            </P>
            <P>
              {'You will win the token if you are the first to claim on the block produced immediately after the timer reaches 0. However, this may mean due to latency you have a better chance submitting your transaction a couple of seconds before the timer reaches 0.'}
            </P>
            <Ul>
              <Li>{'Transaction fee: 0.15 $UST'}</Li>
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
              sizevariant={'large'}
              colorvariant={'secondary'}
              css={css`
                margin-top: 15px;
              `}
            >
              {'CLAIM BONUS'}
            </Button>
          </Form>
        </>
      )}
    </ModalContent>
  )
}

export default ClaimBonus
