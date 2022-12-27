import { useCurrentTimeDate } from 'src/contexts/currentTimeDate.context'

const useCountdown = (startTimeMilliseconds: number, durationMilliseconds: number) => {
  const { currentTimeDate } = useCurrentTimeDate()
  const timeElapsed = currentTimeDate.getTime() - startTimeMilliseconds
  const timeUntilClaim = Math.max(0, durationMilliseconds - timeElapsed)
  const isComplete = timeUntilClaim <= 0

  const secondsUntilClaim = timeUntilClaim / 1000
  const seconds = Math.floor(secondsUntilClaim % 60)
  const minutes = Math.floor(secondsUntilClaim % 3600 / 60)
  const hours = Math.floor(secondsUntilClaim % (3600 * 24) / 3600)
  const days = Math.floor((secondsUntilClaim / (3600 * 24)))
  const countdownView = `${days.toString().padStart(2, '0')}:${hours.toString().padStart(2, '0')}:${minutes
    .toString()
    .padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`

  return {
    isComplete,
    countdownView,
  }
}
export default useCountdown
