import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { QuestConfig } from 'src/models/quest.models'

const useGetObjectiveCompletedCount = (objective_id: number) => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = (): Promise<QuestConfig> => {
    return api!!.contractQuery(
      environmentContext.QUEST_CONTRACT_ADDRESS,
      {
        get_objective_completed_count: {
          objective_id: objective_id,
        },
      },
    )
  }
  const result = useQuery<
  QuestConfig,
  unknown,
  QuestConfig
  >(['questObjectiveCompletedCount'], query, {
    enabled: !!api,
  })
  return { globalObjectiveCount: result?.data }
}

export default useGetObjectiveCompletedCount
