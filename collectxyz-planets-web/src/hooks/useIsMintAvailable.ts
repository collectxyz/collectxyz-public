import useXyzConfig from 'src/hooks/useXyzConfig'
import useXyzNumTokens from 'src/hooks/useXyzNumTokens'
import useXyzTokens from 'src/hooks/useXyzTokens'

const useIsMintAvailable = () => {
  const xyzConfig = useXyzConfig()
  const { result: xyzTokens, count } = useXyzTokens()

  const xyzWalletAmount = count
  const xyzWalletLimit = xyzConfig.data?.wallet_limit
  const xyzNumTokens = useXyzNumTokens()
  const xyzTokenSupply = xyzConfig.data?.token_supply

  const walletMaxReached =
    xyzWalletAmount !== undefined &&
    xyzWalletLimit !== undefined &&
    xyzWalletAmount >= xyzWalletLimit
  const globalMaxReached =
    xyzNumTokens.data !== undefined &&
    xyzTokenSupply !== undefined &&
    xyzNumTokens.data?.count >= xyzTokenSupply
  const publicMintingEnabled =
  !!xyzConfig.data?.public_minting_enabled
  const isMintAvailable =
    xyzWalletAmount !== undefined &&
    xyzWalletLimit !== undefined &&
    xyzNumTokens !== undefined &&
    xyzTokenSupply !== undefined &&
    !walletMaxReached &&
    !globalMaxReached &&
    publicMintingEnabled

  const isDataLoading = xyzTokens.isLoading || xyzNumTokens.isLoading || xyzConfig.isLoading

  return {
    xyzWalletAmount,
    xyzWalletLimit,
    xyzNumTokens,
    xyzTokenSupply,
    walletMaxReached,
    globalMaxReached,
    isMintAvailable,
    publicMintingEnabled,
    isDataLoading,
  }
}

export default useIsMintAvailable
