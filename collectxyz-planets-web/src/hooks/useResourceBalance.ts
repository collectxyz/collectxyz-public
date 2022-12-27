import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

export interface ResourceBalance {
  balance: string
}

const useResourceBalance = (id: string, contractAddress: string, onSuccess?: (data: ResourceBalance) => void) => { // TODO
  const { terraClient, api } = useTerraClient()
  const connectedWallet = useConnectedWallet()
  const environmentContext = useEnvironment()
  const query = (): Promise<ResourceBalance> => {
    return api!!.contractQuery(
      contractAddress,
      {
        balance: {
          xyz_id: id,
        },
      },
    )
  }
  const result = useQuery<
  ResourceBalance,
  unknown,
  ResourceBalance
  >(['resourceBalance', id, contractAddress], query, {
    enabled: !!api && !!connectedWallet,
    onSuccess: onSuccess,
  })

  const amountNumber = result.data !== undefined ? parseInt(result.data.balance) / 1000000 : undefined
  return {
    result,
    amountNumber,
  }
}

export default useResourceBalance
