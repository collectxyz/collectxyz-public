import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

interface AllowanceResponse {
  allowance: string
  expires: string
}
// Allowance { owner_xyz_id: String, owner: String, spender: String },
const useXyzMarketplaceResourceAllowance = (name: string, contractAddress: string) => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const connectedWallet = useConnectedWallet()

  const query = async (): Promise<AllowanceResponse | undefined> => {
    return api!!.contractQuery<AllowanceResponse | null>(
      contractAddress,
      {
        allowance: {
          owner_xyz_id: name,
          owner: connectedWallet?.walletAddress,
          spender: environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
        },
      },
    ).then((response) => !response ? undefined : response)
  }
  const result = useQuery<
  AllowanceResponse | undefined,
  unknown,
  AllowanceResponse | undefined
  >(['marketplaceResourceAllowance', name, contractAddress], query, {
    enabled: api !== undefined,
  })

  return {
    result,
  }
}

export default useXyzMarketplaceResourceAllowance
