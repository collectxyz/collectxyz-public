import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { TaskResponse } from 'src/models/task.models'

const useXyzPlanetTask = (name: string, onSuccess?: (data: TaskResponse | undefined) => void) => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = async (): Promise<TaskResponse | undefined> => {
    return api!!.contractQuery<TaskResponse | null>(
      environmentContext.PLANETS_CONTRACT_ADDRESS,
      {
        get_task_for_nft: {
          xyz_nft_id: name,
        },
      },
    ).then((response) => !response ? undefined : response)
  }
  const result = useQuery<
  TaskResponse | undefined,
  unknown,
  TaskResponse | undefined
  >(['planetTasks', name], query, {
    enabled: api !== undefined,
    onSuccess: onSuccess,
  })

  return {
    result,
  }
}

export default useXyzPlanetTask
