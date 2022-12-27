import React from 'react'
import { Link } from 'react-router-dom'
import PlanetCardContent from 'src/components/PlanetCardContent'
import useCountdown from 'src/hooks/useCountdown'
import useXyzPlanets from 'src/hooks/useXyzPlanets'
import { XyzResponse } from 'src/models/xyz.models'
import { css } from 'styled-components'
import P from './P'

interface XyzCardProps {
  xyzResponse: XyzResponse
}

const XyzCard: React.FC<XyzCardProps> = ({ xyzResponse }) => {
  const { result: planetsResult } = useXyzPlanets(
    xyzResponse.extension.coordinates,
  )

  const { isComplete: moveComplete, countdownView } = useCountdown(
    0,
    xyzResponse.extension.arrival / 1000000,
  )

  return (
    <Link to={`/xyz/${xyzResponse.name.split('#')[1]}`}>
      <div
        css={css`
          box-shadow: 0 0 5px 2px lightgray;
          position: relative;
          aspect-ratio: 1/1;
          width: 195px;
          height: 195px;
          transition: box-shadow 200ms ease-out;
          background-color: black;
          opacity: ${moveComplete ? 1 : 0.6};
          pointer-events: ${moveComplete ? undefined : 'none'};
          box-shadow: ${moveComplete
      ? '0 0 5px 2px lightgray'
      : '0 0 3px 1px lightgray'};
      &:hover {
        box-shadow: 0 0 3px 1px lightgray;
      }
        `}
      >
        <div
          css={css`
            position: absolute;
            top: 10px;
            left: 10px;
            display: flex;
            flex-direction: column;
            white-space: nowrap;
            z-index: 1;
          `}
        >
          <P
            css={css`
              color: lightgray;
              font-size: 14px;
              white-space: nowrap;
            `}
          >
            {`[${xyzResponse.extension.coordinates.x}, ${xyzResponse.extension.coordinates.y}, ${xyzResponse.extension.coordinates.z}]`}
          </P>

          <P
            css={css`
              color: darkGrey;
              font-size: 12px;
            `}
          >
            {xyzResponse.name}
          </P>
        </div>
        {!moveComplete && (
          <div
            css={css`
              position: absolute;
              top: 50%;
              left: 50%;
              transform: translate(-50%, -50%);
              display: flex;
              flex-direction: column;
              align-items: center;
              white-space: nowrap;
            `}
          >
            {xyzResponse.extension.prev_coordinates && (
              <>
                <P
                  css={css`
                    color: darkGrey;
                    font-size: 12px;
                  `}
                >
                  {`Relocating from:`}
                </P>
                <P
                  css={css`
                    color: darkGrey;
                    font-size: 12px;
                  `}
                >
                  {`[${xyzResponse.extension.prev_coordinates.x}, ${xyzResponse.extension.prev_coordinates.y}, ${xyzResponse.extension.prev_coordinates.z}]`}
                </P>
              </>
            )}
            <P
              css={css`
                color: darkGrey;
                font-size: 12px;
              `}
            >
              {countdownView}
            </P>
          </div>
        )}
        {moveComplete && (
          <>
            <PlanetCardContent
              xyzResponse={xyzResponse}
              planets={planetsResult.data}
            ></PlanetCardContent>
          </>
        )}
      </div>
    </Link>
  )
}

export default XyzCard
