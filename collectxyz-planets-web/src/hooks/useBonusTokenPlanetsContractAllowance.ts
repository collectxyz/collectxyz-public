import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

export interface BonusTokenAllowance {
  allowance: string
}

const useBonusTokenPlanetsContractAllowance = (onSuccess?: (data: BonusTokenAllowance) => void) => { // TODO
  const { terraClient, api } = useTerraClient()
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const query = (): Promise<BonusTokenAllowance> => {
    return api!!.contractQuery(
      environmentContext.BONUS_TOKEN_CONTRACT_ADDRESS,
      {
        allowance: {
          owner: connectedWallet!!.walletAddress,
          spender: environmentContext.PLANETS_CONTRACT_ADDRESS,
        },
      },
    )
  }
  const result = useQuery<
  BonusTokenAllowance,
  unknown,
  BonusTokenAllowance
  >(['bonusTokenPlanetsContractAllowance'], query, {
    enabled: !!api && !!connectedWallet,
    onSuccess: onSuccess,
  })

  return { result }
}

export default useBonusTokenPlanetsContractAllowance
