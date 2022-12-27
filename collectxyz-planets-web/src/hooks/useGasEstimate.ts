import { Msg, StdFee } from '@terra-money/terra.js'
import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useQuery } from 'react-query'
import { useTerraClient } from 'src/hooks/useTerraClient'

const useGasEstimate = (msgs: Msg[]) => {
  const { terraClient } = useTerraClient()
  const connectedWallet = useConnectedWallet()
  const query = async (): Promise<StdFee> => {
    return terraClient!!.tx.estimateFee(connectedWallet!!.walletAddress, msgs, {
      feeDenoms: ['uusd'],
    })
  }
  const result = useQuery<StdFee, unknown, StdFee>(
    ['gasEstimate', msgs],
    query,
    {
      enabled: !!terraClient && !!connectedWallet,
      staleTime: 0,
    },
  )

  const gasDollars = result.data !== undefined
    ? (parseInt(
      result.data?.amount
        .toString()
        .slice(0, result.data.amount.toString().length - 4),
    ) / 1000000).toPrecision(2)
    : undefined

  return {
    result,
    gasDollars,

  }
}

export default useGasEstimate
