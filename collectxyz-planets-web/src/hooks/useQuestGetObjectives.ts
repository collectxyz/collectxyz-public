import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { Objective } from 'src/models/quest.models'

const useQuestGetObjectives = () => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = (): Promise<Objective[]> => {
    return api!!.contractQuery(
      environmentContext.QUEST_CONTRACT_ADDRESS,
      {
        get_objectives: {},
      },
    )
  }
  const result = useQuery<
  Objective[],
  unknown,
  Objective[]
  >(['questObjectives'], query, {
    enabled: !!api,
  })

  const count = result.data !== undefined
    ? result.data.length
    : 0

  return { objectives: result?.data, count }
}

export default useQuestGetObjectives
