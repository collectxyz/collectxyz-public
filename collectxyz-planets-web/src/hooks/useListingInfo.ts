import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { ListingResponse } from 'src/models/listing.models'

const useListingInfo = (id: number) => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = ({ pageParam = 0 }): Promise<ListingResponse> => {
    return api!!.contractQuery(
      environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
      {
        listing_info: {
          listing_id: id,
        },
      },
    )
  }
  const result = useQuery<
  ListingResponse,
  unknown,
  ListingResponse
  >(['listing', id], query, {
    enabled: !!api,
  })
  return result
}

export default useListingInfo
