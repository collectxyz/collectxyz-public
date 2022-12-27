import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { ResourceIdentifier } from 'src/models/planet.models'

interface RichnessThreshold {
  level_one: number
  level_two: number
  level_three: number
  level_four: number
  level_five: number
}
interface ResourceGenerationInfo {
  resource_identifier: ResourceIdentifier
  resource_contract_address: string
  appearance_probability: number
  richness_thresholds: RichnessThreshold
}
interface PlanetsConfig {
  boost_per_bonus_token: number
  core_resource_generation_info: ResourceGenerationInfo[]
  cw20_bonus_token_contract: string
  max_number_of_bonus_tokens: number
  maximum_planets_per_coord: number
  probability_of_discovery: number
  randomness_contract_address: string
  required_seconds: number
  resource_generation_info: ResourceGenerationInfo[]
  discovery_task_expiration_window_seconds: number
  xyz_nft_contract_address: string
}

const usePlanetsConfig = () => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = ({ pageParam = 0 }): Promise<PlanetsConfig> => {
    return api!!.contractQuery(
      environmentContext.PLANETS_CONTRACT_ADDRESS,
      {
        get_current_config: {
        },
      },
    )
  }
  const result = useQuery<
  PlanetsConfig,
  unknown,
  PlanetsConfig
  >(['planetsConfig'], query, {
    enabled: !!api,
  })

  return result
}

export default usePlanetsConfig
