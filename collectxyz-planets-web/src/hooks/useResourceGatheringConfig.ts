import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

interface ResourceGatheringConfig {
  planet_contract_address: string
  randomness_contract_address: string
  xyz_nft_contract_address: string
  resource_gathering_info: Record<string, unknown>
  gather_task_duration_seconds: number
  gather_task_expiration_seconds: number
}

const useResourceGatheringConfig = () => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = ({ pageParam = 0 }): Promise<ResourceGatheringConfig> => {
    return api!!.contractQuery(
      environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS,
      {
        get_current_config: {
        },
      },
    )
  }
  const result = useQuery<
  ResourceGatheringConfig,
  unknown,
  ResourceGatheringConfig
  >(['resourceGatheringConfig'], query, {
    enabled: !!api,
  })

  return result
}

export default useResourceGatheringConfig
