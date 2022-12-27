import { useEffect, useState } from 'react'
import { useQuery } from 'react-query'
import { useTerraClient } from 'src/hooks/useTerraClient'

const usePollTxHash = (txhash: string) => {
  const [refetchInterval, setRefetchInterval] = useState<number | false>(false)
  const { terraClient, api } = useTerraClient()

  const tx = () => terraClient?.tx.txInfo(txhash)
  const { data } = useQuery(txhash, tx, {
    refetchInterval,
    enabled: !!txhash && terraClient !== undefined,
  })
  //
  const height = data && data.height
  //
  useEffect(() => {
    if (height) {
      setRefetchInterval(false)
    } else {
      setRefetchInterval(2000)
    }
  }, [height])
  //
  return data
}

export default usePollTxHash
