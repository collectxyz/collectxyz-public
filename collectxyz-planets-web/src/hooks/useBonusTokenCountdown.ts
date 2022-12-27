import useCountdown from 'src/hooks/useCountdown'
import useLatestRand from 'src/hooks/useLatestRand'
import useRandomnessConfig from 'src/hooks/useRandomnessConfig'

const useBonusTokenCountdown = () => {
  const randomnessConfig = useRandomnessConfig()
  const latestRand = useLatestRand()

  const currentDate = new Date().getTime()
  const windowLengthDurationMilliseconds =
    randomnessConfig !== undefined
      ? randomnessConfig.config.time_slot_nanos / 1000000
      : 1000 * 60 * 100
  const mostRecentClaimWindowStartMilliseconds =
    currentDate - (currentDate % windowLengthDurationMilliseconds)
  const lastClaimMilliseconds = latestRand.data?.slot
    ? latestRand.data?.slot / 1000000
    : undefined
  const renderCountdown =
    latestRand.isLoading || latestRand.data === undefined ||
    lastClaimMilliseconds === mostRecentClaimWindowStartMilliseconds
  const { countdownView } = useCountdown(
    mostRecentClaimWindowStartMilliseconds,
    windowLengthDurationMilliseconds,
  )
  return { renderCountdown, countdownView}
}

export default useBonusTokenCountdown
