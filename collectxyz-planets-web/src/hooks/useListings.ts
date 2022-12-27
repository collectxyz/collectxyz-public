import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useInfiniteQuery, useQueryClient } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { ListingResponse } from 'src/models/listing.models'

export interface ListingsResponse {
  listings: ListingResponse[]
}

export interface UseListingsParams {
  lister_xyz_id?: string
  prices?: string[]
  resources?: string[]
  include_inactive?: boolean
  ascending?: boolean
  start_after?: number
  limit?: number
}

const limit = 16
const useListings = (params: UseListingsParams) => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const connectedWallet = useConnectedWallet()

  const query = async ({pageParam = undefined}): Promise<ListingsResponse> => {
    const initialResult = await api!!.contractQuery<ListingsResponse>(
      environmentContext.MARKETPLACE_CONTRACT_ADDRESS,
      {
        listings: {
          limit,
          ...params,
          start_after: pageParam,
        },
      },
    )

    return initialResult
  }
  const result = useInfiniteQuery<
  ListingsResponse,
  unknown,
  ListingsResponse
  >(['listings', params], query, {
    enabled: api !== undefined,
    staleTime: 0,
    refetchInterval: 10000,
    onSuccess: (data) => {
      data.pages.forEach((page) => {
        page.listings.forEach((listing) => {
          queryClient.setQueryData(['listing', listing.listing_id], listing)
        })
      })
    },
    getNextPageParam: (lastPage) => {
      return lastPage.listings.length === limit ? lastPage.listings[lastPage.listings.length - 1].listing_id : undefined
    },
  })

  return {
    result,
    limit,
  }
}

export default useListings
