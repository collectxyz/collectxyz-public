import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

export interface BonusTokenBalance {
  balance: string
}

const useBonusTokenBalance = (onSuccess?: (data: BonusTokenBalance) => void) => { // TODO
  const { terraClient, api } = useTerraClient()
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const query = (): Promise<BonusTokenBalance> => {
    return api!!.contractQuery(
      environmentContext.BONUS_TOKEN_CONTRACT_ADDRESS,
      {
        balance: {
          address: connectedWallet!!.walletAddress,
        },
      },
    )
  }
  const result = useQuery<
  BonusTokenBalance,
  unknown,
  BonusTokenBalance
  >(['bonusTokenBalance'], query, {
    enabled: !!api && !!connectedWallet,
    onSuccess: onSuccess,
  })

  return { result }
}

export default useBonusTokenBalance
