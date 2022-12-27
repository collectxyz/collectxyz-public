import React, { useEffect, useRef, useState } from 'react'
import { fonts } from 'src/styles/constants'
import styled, { css } from 'styled-components'

const LoadingContainer = styled.span`
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 304px;
  grid-row-gap: 10px;
`

const Row = styled.span`
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 240px;
`
const Indicator = styled.span`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  font-family: ${fonts.highlight};
  height: 35px;
  width: 35px;
`

interface LoadingProps {
  isLoading?: boolean
  color?: string
}
const Loading: React.FC<LoadingProps> = (props) => {
  const [spinnerState, setSpinnerState] = useState(0)
  const interval = useRef<NodeJS.Timer>()
  useEffect(() => {
    interval.current = setInterval(() => {
      setSpinnerState((x) => {
        return (x + 1) % 4
      })
    }, 750)
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
    < LoadingContainer css={css`
    color: ${props.color};
    `}>
      <Row>
        <Indicator>{props.isLoading ? currentSymbol : '/'}</Indicator>
        <Indicator>{props.isLoading ? currentSymbol : '-'}</Indicator>
        <Indicator>{props.isLoading ? currentSymbol : '-'}</Indicator>
        <Indicator>{props.isLoading ? currentSymbol : '\\'}</Indicator>
      </Row>
      <Row>
        <Indicator>{props.isLoading ? currentSymbol : '|'}</Indicator>
        {props.children}
        <Indicator>{props.isLoading ? currentSymbol : '|'}</Indicator>
      </Row>
      <Row>
        <Indicator>{props.isLoading ? currentSymbol : '\\'}</Indicator>
        <Indicator>{props.isLoading ? currentSymbol : '-'}</Indicator>
        <Indicator>{props.isLoading ? currentSymbol : '-'}</Indicator>
        <Indicator>{props.isLoading ? currentSymbol : '/'}</Indicator>
      </Row>
    </LoadingContainer>
  )
}

export default Loading
