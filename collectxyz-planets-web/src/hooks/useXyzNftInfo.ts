import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { XyzResponse } from 'src/models/xyz.models'

const useXyzNftInfo = (name: string, onSuccess?: (data: XyzResponse) => void) => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = ({ pageParam = 0 }): Promise<XyzResponse> => {
    return api!!.contractQuery(
      environmentContext.XYZ_CONTRACT_ADDRESS,
      {
        xyz_nft_info: {
          token_id: name, // TODO
        },
      },
    )
  }
  const result = useQuery<
  XyzResponse,
  unknown,
  XyzResponse
  >(['xyzs', name], query, {
    enabled: !!api,
    onSuccess: onSuccess,
  })
  return result
}

export default useXyzNftInfo
