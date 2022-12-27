import { useQuery, useQueryClient } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import useXyzTokens from 'src/hooks/useXyzTokens'
import { TaskResponse } from 'src/models/task.models'

const useResourceGatheringTasks = () => { // TODO
  const {
    sortedTokens,
  } = useXyzTokens()
  const { api } = useTerraClient()
  const environmentContext = useEnvironment()
  const queryClient = useQueryClient()

  const query = async (): Promise<TaskResponse[]> => {
    const promises = sortedTokens!!.map((token) => {
      return api!!.contractQuery<TaskResponse | null>(
        environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS,
        {
          get_task_for_nft: {
            xyz_nft_id: token.name,
          },
        },
      ).then((response) => !response ? undefined : response)
    })
    const a = (await Promise.all(promises)).filter((entry) => !!entry)
    return a as TaskResponse[]
  }
  const result = useQuery<
  TaskResponse[],
  unknown,
  TaskResponse[]
  >(['resourceGatheringTasks'], query, {
    onSuccess: (data) => {
      data.forEach((task) => {
        queryClient.setQueryData(['resourceGatheringTasks', task.nft_token_id], task)
      })
    },
    enabled: api !== undefined && sortedTokens !== undefined,
  })

  return {
    result,
  }
}

export default useResourceGatheringTasks
