import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'

const useXyzNumTokens = () => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = ({ pageParam = 0 }): Promise<{count: number}> => {
    return api!!.contractQuery(
      environmentContext.XYZ_CONTRACT_ADDRESS,
      {
        num_tokens: {
        },
      },
    )
  }
  const result = useQuery<
  {count: number},
  unknown,
  {count: number}
  >(['num_tokens'], query, {
    enabled: !!api,
  })

  return result
}

export default useXyzNumTokens
