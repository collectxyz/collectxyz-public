import { useConnectedWallet } from '@terra-money/wallet-provider'
import React from 'react'
import LoadingIndicator from 'src/components/LoadingIndicator'
import PlanetOrbit from 'src/components/PlanetOrbit'
import { PlanetModel } from 'src/models/planet.models'
import { XyzResponse } from 'src/models/xyz.models'
import { css } from 'styled-components'

interface PlanetCardContentProps {
  xyzResponse: XyzResponse
  planets?: PlanetModel[]
  className?: string
}
const PlanetCardContent: React.FC<PlanetCardContentProps> = (props) => {
  const connectedWallet = useConnectedWallet()

  return (
    <div
      className={props.className}
      css={css`
        position: absolute;
        width: 100%;
        height: 100%;
        transition: opacity 700ms ease-out, transform 700ms linear;
        z-index: 0;
      `}
    >
      <p
        css={css`
          transition: opacity 700ms ease-out;
          opacity: ${props.planets === undefined ? 1 : 0};
          pointer-events: ${props.planets === undefined ? undefined : 'none'};
          color: darkGrey;
          display: flex;
          width: 100%;
          height: 100%;
          align-items: center;
          justify-content: center;
        `}
      >
        <LoadingIndicator></LoadingIndicator>
      </p>
      <div
        css={css`
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          transition: opacity 700ms ease-out;
          opacity: ${props.planets !== undefined ? 1 : 0};
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          font-size: 12px;
        `}
      >
        {props.planets?.length === 0 && connectedWallet?.walletAddress === props.xyzResponse.owner && (
          <>
            <p
              css={css`
                color: darkGrey;
              `}
            >
              {'View details'}
            </p>
          </>
        )}
        {props.planets !== undefined && props.planets.length > 0 && (
          <>
            <div
              css={css`
                position: absolute;
                width: 100%;
                height: 100%;
                transform: rotateX(75deg);
                transform-style: preserve-3d;
              `}
            >
              <PlanetOrbit
                pixelHeight={30}
                animationLength={32}
                orbitSize={30}
                planet={props.planets[0]}
              ></PlanetOrbit>
              <PlanetOrbit
                pixelHeight={12}
                animationLength={12}
                orbit
                orbitSize={60}
                planet={props.planets[1]}
              ></PlanetOrbit>
              <PlanetOrbit
                pixelHeight={20}
                animationLength={40}
                orbit
                orbitSize={120}
                planet={props.planets[2]}
              ></PlanetOrbit>
            </div>
          </>
        )}
      </div>
    </div>
  )
}

export default PlanetCardContent
