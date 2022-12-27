import React, { useMemo } from 'react'
import { PlanetModel } from 'src/models/planet.models'
import { generateRandFunction } from 'src/utils'
import styled, { css, keyframes } from 'styled-components'

const invert = keyframes`
  0% {
    transform: rotateX(-90deg) rotateY(360deg) rotateZ(0deg); }

  100% {
    transform: rotateX(-90deg) rotateY(0deg) rotateZ(0deg); }
`
const PlanetContainer = styled.div<{
  animationLength: number
  orbit?: boolean
  pixelHeight: number
}>`
  height: ${(props) => props.pixelHeight}px;
  width: ${(props) => props.pixelHeight}px;
  box-shadow: 0px 0px 2px 1px darkGray;
  aspect-ratio: 1 / 1;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  transform-style: preserve-3d;
  position: absolute;
  transform: rotateX(-90deg);
  animation: ${invert}
    ${(props) => (props.orbit ? props.animationLength : '0s')}s linear infinite;
`

const bgAnimation = (pixelWidth: number) => keyframes`
  from {
    transform: translate(0px);
  }
  to {
    transform: translate(-${pixelWidth}px);
  }
`
const PlanetBackground = styled.div<{
  pixelWidth: number
  animationLength: number
}>`
  width: 1px;
  height: 1px;
  animation: ${(props) => bgAnimation(props.pixelWidth)}
    ${(props) => props.animationLength / 2}s linear infinite;
  // filter: blur(0.3px);
`

const PlanetOcclusion = styled.div`
  position: absolute;
  border-radius: 50%;
  box-shadow: inset 1px 0px 1px 1px darkGray;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 1;
  background-image: linear-gradient(
    to bottom right,
    rgba(0, 0, 0, 0) 0%,
    rgba(0, 0, 0, 0) 30%,
    rgba(0, 0, 0, 0.9) 75%,
    rgba(0, 0, 0, 1) 100%
  );
`

interface PlanetProps {
  pixelHeight: number
  orbit?: boolean
  animationLength: number
  planet: PlanetModel
}
const Planet: React.FC<PlanetProps> = ({ pixelHeight, ...props }) => {
  const pixelWidth = pixelHeight * 3
  const rand = useMemo(() => generateRandFunction(), [])
  const randArray = useMemo(
    () =>
      Array.from(Array(pixelHeight * pixelWidth).keys()).map(() => rand()),
    [],
  )

  const colorRandArray = useMemo(
    () =>
      randArray.map(
        (num, i) => {
          return props.planet.primaryColorPalette[Math.floor(num * props.planet.primaryColorPalette.length)]
        },
      ),
    [],
  )
  const boxShadowArray = useMemo(
    () =>
      Array.from(Array(pixelHeight * pixelWidth).keys()).map(
        (el, i) =>
          `${(i % (pixelWidth)) - pixelHeight / 2}px ${
            Math.floor(i / (pixelWidth)) - pixelHeight / 2
          }px ${props.planet.gasBlurMultiplier}px 1px ${colorRandArray[i]}`,
      ),
    [],
  )
  const extendedBoxShadowArray = useMemo(
    () =>
      Array.from(Array(pixelHeight * pixelWidth).keys()).map(
        (el, i) =>
          `${(i % (pixelWidth)) - pixelHeight / 2 + pixelWidth}px ${
            Math.floor(i / (pixelWidth)) - pixelHeight / 2
          }px ${props.planet.gasBlurMultiplier}px 1px ${colorRandArray[i]}`,
      ),
    [],
  )
  return (
    <PlanetContainer
      animationLength={props.animationLength}
      pixelHeight={pixelHeight}
      orbit={props.orbit}
    >
      <PlanetOcclusion></PlanetOcclusion>
      <PlanetBackground
        animationLength={props.animationLength}
        pixelWidth={pixelWidth}
        css={css`
          box-shadow: ${[...boxShadowArray, ...extendedBoxShadowArray].join()};
          background: ${colorRandArray[0]};
        `}
      ></PlanetBackground>
    </PlanetContainer>
  )
}
export default Planet

