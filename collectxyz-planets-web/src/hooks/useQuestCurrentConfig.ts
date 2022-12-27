import { useQuery } from 'react-query'
import { useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { QuestConfig } from 'src/models/quest.models'

const useQuestCurrentConfig = () => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const query = (): Promise<QuestConfig> => {
    return api!!.contractQuery(
      environmentContext.QUEST_CONTRACT_ADDRESS,
      {
        current_config: {},
      },
    )
  }
  const result = useQuery<
  QuestConfig,
  unknown,
  QuestConfig
  >(['questCurrentConfig'], query, {
    enabled: !!api,
  })
  return { quest: result?.data }
}

export default useQuestCurrentConfig
