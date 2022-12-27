import React from 'react'
import Planet from 'src/components/Planet'
import { PlanetModel } from 'src/models/planet.models'
import { css, keyframes } from 'styled-components'

const orbit = keyframes`
  0% {
    transform: rotateZ(0deg); }

  100% {
    transform: rotateZ(-360deg); } 
`

interface PlanetOrbitProps {
  pixelHeight: number
  orbit?: boolean
  animationLength: number
  orbitSize: number
  planet?: PlanetModel
}
const PlanetOrbit: React.FC<PlanetOrbitProps> = (props) => {
  return (
    <div
      css={css`
        width: ${props.orbitSize}px;
        height: ${props.orbitSize}px;
        border-radius: 50%;
        transform-style: preserve-3d;
        z-index: 0;
        position: absolute;
        top: 50%;
        border: ${props.orbit
      ? '1px solid rgba(255, 255, 255, 0.2)'
      : undefined};
        left: 50%;
        margin-top: -${props.orbitSize / 2}px;
        margin-left: -${props.orbitSize / 2}px;
        animation: ${props.orbit ? orbit : 'none'} ${props.animationLength}s
          linear infinite;
      `}
    >
      {props.planet && (
        <Planet
          pixelHeight={props.pixelHeight}
          animationLength={props.animationLength}
          orbit={props.orbit}
          planet={props.planet}
        ></Planet>
      )}
    </div>
  )
}

export default PlanetOrbit
