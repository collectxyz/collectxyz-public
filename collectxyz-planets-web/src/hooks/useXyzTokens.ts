import { useConnectedWallet } from '@terra-money/wallet-provider'
import { useMemo } from 'react'
import { useQuery, useQueryClient } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { XyzResponse } from 'src/models/xyz.models'

export interface XyzTokensWithCoords {
  tokens: XyzResponse[]
}

const limit = 25
const useXyzTokens = (onSuccess?: (data: XyzTokensWithCoords) => void) => { // TODO
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()
  const connectedWallet = useConnectedWallet()

  const query = async ({ pageParam = '0' }): Promise<XyzTokensWithCoords> => {
    const initialResult = await api!!.contractQuery<XyzTokensWithCoords>(
      environmentContext.XYZ_CONTRACT_ADDRESS,
      {
        xyz_tokens: {
          owner: connectedWallet?.terraAddress,
          limit: limit,
          start_after: `${pageParam}`,
        },
      },
    )

    return {
      tokens: initialResult.tokens.length < limit
        ? initialResult.tokens
        : [
          ...initialResult.tokens,
          ...(await query({ pageParam: `${initialResult.tokens[initialResult.tokens.length - 1].name}` })).tokens,
        ],
    }
  }
  const result = useQuery<
  XyzTokensWithCoords,
  unknown,
  XyzTokensWithCoords
  >(['xyzs'], query, {
    onSuccess: (data) => {
      data.tokens.forEach((token) => {
        queryClient.setQueryData(['xyzs', token.name], token)
      })
      onSuccess?.(data)
    },
    enabled: api !== undefined,
  })

  const count = result.data?.tokens.length

  const sortedTokens = useMemo(() => (
    result.data?.tokens.sort((a, b) => (
      parseInt(b.name.split('#')[1]) - parseInt(a.name.split('#')[1])
    ))
  ), [result])

  return {
    result,
    limit,
    count,
    sortedTokens,
  }
}

export default useXyzTokens
