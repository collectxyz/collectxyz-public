import { useWallet } from '@terra-money/wallet-provider'
import { useMemo } from 'react'

const useTerraFinderUrl = (objectType: string, path: string) => {
  const { network } = useWallet()
  const url = useMemo(() => {
    return `https://finder.terra.money/${network.chainID}/${objectType}/${path}`
  }, [objectType, path])
  return url
}

export default useTerraFinderUrl
