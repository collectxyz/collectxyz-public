import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { Coin } from 'src/models/quest.models'

const useQuestGetReward = (xyz_id: string) => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()

  const query = (): Promise<Coin> => {
    return api!!.contractQuery(
      environmentContext.QUEST_CONTRACT_ADDRESS,
      {
        get_reward: {
          xyz_id: xyz_id,
        },
      },
    )
  }
  const result = useQuery<
  Coin,
  unknown,
  Coin
  >(['questReward', xyz_id], query, {
    enabled: !!api,
  })
  return { reward: result?.data }
}

export default useQuestGetReward
