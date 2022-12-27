import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

interface RandomnessConfig {
  config: {

    // seeds: Vec<Seed>,
    time_slot_nanos: number
    expiry_nanos: number
    cw20_contract: string
  }
}

const useRandomnessConfig = () => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = (): Promise<RandomnessConfig> => {
    return api!!.contractQuery(
      environmentContext.RANDOMNESS_CONTRACT_ADDRESS,
      {
        config: {
        },
      },
    )
  }
  const { data, error, status, isSuccess } = useQuery<
  RandomnessConfig,
  unknown,
  RandomnessConfig
  >(['randomnessConfig'], query, {
    enabled: !!api,
  })

  return data
}

export default useRandomnessConfig
