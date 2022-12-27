import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { PlanetModel, PlanetResponse } from 'src/models/planet.models'

const limit = 25
const useXyzPlanets = (coordinates?: { x: number, y: number, z: number }, onSuccess?: (data: PlanetModel[]) => void) => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = async ({ pageParam = '0' }): Promise<PlanetModel[]> => {
    const initialResult = await api!!.contractQuery<{claimed_planets: PlanetResponse[]}>(
      environmentContext.PLANETS_CONTRACT_ADDRESS,
      {
        get_planets_for_coords: {
          coordinates: coordinates,
          limit: limit,
          start_after: `${pageParam}`,
        },
      },
    )

    return initialResult.claimed_planets.length < limit
      ? initialResult.claimed_planets.map((planet) => (new PlanetModel(planet)))
      : [
        ...initialResult.claimed_planets.map((planet) => (new PlanetModel(planet))),
        ...(await query({ pageParam: `${initialResult.claimed_planets[initialResult.claimed_planets.length - 1].planet_id}` })),
      ]
  }
  const result = useQuery<
  PlanetModel[],
  unknown,
  PlanetModel[]
  >(['planets', coordinates], query, {
    enabled: api !== undefined && coordinates !== undefined,
    onSuccess: onSuccess,
  })

  return {
    result,
  }
}

export default useXyzPlanets
