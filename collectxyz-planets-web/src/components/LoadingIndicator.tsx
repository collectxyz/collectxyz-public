import React, { useEffect, useRef, useState } from 'react'
import { fonts } from 'src/styles/constants'
import styled from 'styled-components'

const LoadingIndicatorContainer = styled.span`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  font-family: ${fonts.highlight};
  height: 35px;
  width: 35px;
`

interface LoadingIndicatorProps {
  className?: string
  color?: string
  speed?: number
}
const LoadingIndicator: React.FC<LoadingIndicatorProps> = (props) => {
  const [spinnerState, setSpinnerState] = useState(0)
  const interval = useRef<NodeJS.Timer>()
  useEffect(() => {
    interval.current = setInterval(() => {
      setSpinnerState((x) => {
        return (x + 1) % 4
      })
    }, 750 || props.speed)
    return () => {
      if (interval.current !== undefined) {
        clearInterval(interval.current)
      }
    }
  }, [])
  const currentSymbol = spinnerState === 0
    ? '/'
    : spinnerState === 1 ? '-'
      : spinnerState === 2 ? '\\'
        : '|'
  return (
    <LoadingIndicatorContainer className={props.className}>{currentSymbol}</LoadingIndicatorContainer>
  )
}

export default LoadingIndicator
