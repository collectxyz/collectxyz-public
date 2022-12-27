import { useQuery, useQueryClient } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import useXyzTokens from 'src/hooks/useXyzTokens'
import { PlanetModel, PlanetResponse } from 'src/models/planet.models'

export interface XyzPlanets {
  claimed_planets: PlanetResponse[]
}

export type UsePlanetsReturn = Record<string, PlanetModel[]>
const usePlanets = () => { // TODO
  const {
    sortedTokens,
  } = useXyzTokens()
  const { api } = useTerraClient()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  const query = async (): Promise<UsePlanetsReturn> => {
    const promises = sortedTokens!!.map((token) => {
      return api!!.contractQuery<XyzPlanets>(
        environmentContext.PLANETS_CONTRACT_ADDRESS,
        {
          get_planets_for_coords: {
            coordinates: token.extension.coordinates,
            limit: 25,
          },
        },
      ).then((response) => (response.claimed_planets.length > 0 ? {
        [token.name]: response.claimed_planets.map((response) => (
          new PlanetModel(response)
        )),
      } : {}))
        .catch(() => {
          return undefined
        })
    })
    const a = (await Promise.all(promises))
      .filter((entry) => entry !== undefined)
      .reduce((acc: UsePlanetsReturn, curr) => (
        {
          ...acc,
          ...curr,
        }
      ), {})
    return a as UsePlanetsReturn
  }

  const result = useQuery<
  UsePlanetsReturn,
  unknown,
  UsePlanetsReturn
  >(['planets'], query, {
    onSuccess: (data) => {
      // data.forEach((task) => {
      //   queryClient.setQueryData(['planets', task.nft_token_id], task)
      // })
    },
    enabled: api !== undefined && sortedTokens !== undefined,
  })

  const count = result.data !== undefined
    ? Object.values(result.data)?.reduce((acc, curr) => (
      acc + curr.length
    ), 0)
    : undefined
  return {
    result,
    count,
  }
}

export default usePlanets
