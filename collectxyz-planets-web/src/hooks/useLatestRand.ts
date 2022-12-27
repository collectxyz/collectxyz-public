import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

interface LatestRand {
  slot: number
  rand: unknown[]
}

const useLatestRand = () => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = (): Promise<LatestRand> => {
    return api!!.contractQuery(
      environmentContext.RANDOMNESS_CONTRACT_ADDRESS,
      {
        latest_rand: {
        },
      },
    )
  }
  const result = useQuery<
  LatestRand,
  unknown,
  LatestRand
  >(['latestRand'], query, {
    enabled: !!api,
    refetchInterval: 30000,
  })

  return result
}

export default useLatestRand
