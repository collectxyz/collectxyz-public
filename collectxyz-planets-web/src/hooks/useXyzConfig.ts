import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

interface XyzConfig {
  base_move_fee: {
    denom: string
    amount: string
  }
  base_move_nanos: number
  mint_fee: {
    denom: string
    amount: string
  }
  move_fee_per_step: string
  move_nanos_per_step: number
  // owner: string
  token_supply: number
  wallet_limit: number
  public_minting_enabled: boolean
}

const useXyzConfig = () => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = ({ pageParam = 0 }): Promise<XyzConfig> => {
    return api!!.contractQuery(
      environmentContext.XYZ_CONTRACT_ADDRESS,
      {
        config: {
        },
      },
    )
  }
  const result = useQuery<
  XyzConfig,
  unknown,
  XyzConfig
  >(['xyzConfig'], query, {
    enabled: !!api,
  })

  return result
}

export default useXyzConfig
