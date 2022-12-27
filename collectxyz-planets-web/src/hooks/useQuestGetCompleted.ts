import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { CompleteObjective } from 'src/models/quest.models'

const useQuestGetCompleted = (xyz_id: string) => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = (): Promise<Array<CompleteObjective>> => {
    return api!!.contractQuery(
      environmentContext.QUEST_CONTRACT_ADDRESS,
      {
        get_completed: {
          xyz_id: xyz_id,
        },
      },
    )
  }
  const result = useQuery<
  Array<CompleteObjective>,
  unknown,
  Array<CompleteObjective>
  >(['questCompleted', xyz_id], query, {
    enabled: !!api,
  })

  const count = result.data !== undefined
  ? result.data.length
  : 0

  return { completedObjectives: result?.data, count }
}

export default useQuestGetCompleted
