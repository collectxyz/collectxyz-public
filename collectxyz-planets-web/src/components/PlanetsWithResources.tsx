import React from 'react'
import Fieldset from 'src/components/Fieldset'
import Planet from 'src/components/Planet'
import PlanetCardContent from 'src/components/PlanetCardContent'
import { PlanetModel } from 'src/models/planet.models'
import { XyzResponse } from 'src/models/xyz.models'
import { mediaDown } from 'src/styles/breakpoints'
import { css } from 'styled-components'

interface PlanetsWithResourcesProps {
  planets: PlanetModel[]
  xyzResponse: XyzResponse
  className?: string
}
const PlanetsWithResources: React.FC<PlanetsWithResourcesProps> = ({
  xyzResponse,
  planets,
  ...props
}) => {
  return (
    <div
      className={props.className}
      css={css`
        display: flex;
        flex-direction: column;
        grid-gap: 25px;
      `}
    >
      {' '}
      <div
        css={css`
          position: relative;
          aspect-ratio: 1/1;
          flex-shrink: 0;
          transition: transform 600ms ease-in-out;
          box-shadow: 0 0 5px 2px lightgray;
              ${mediaDown('md')`
              width: 200px;
              height: 200px;
              `};
        `}
      >
        <PlanetCardContent
          xyzResponse={xyzResponse}
          planets={planets}
          css={css`
            transform: scale(1.2);
          `}
        ></PlanetCardContent>
      </div>
      <Fieldset
        css={css`
        
              ${mediaDown('md')`
              width: 200px;
              `};
       `}
        legend={`Planets (${planets.length}/3 discovered)`}
        items={planets.map((planet, j) => ({
          label: (
            <div
              css={css`
                display: flex;
                margin-right: 10px;
              `}
            >
              <div
                css={css`
                  transform: rotateX(75deg);
                  transform-style: preserve-3d;
                  width: 16px;
                  height: 16px;
                  margin-right: 8px;
                `}
              >
                <Planet planet={planet} pixelHeight={16} animationLength={32} />
              </div>

              <p
                css={css`
                  font-size: 14px;
                  white-space: nowrap;
                `}
              >
                {`#${j + 1}`}
              </p>
            </div>
          ),
          value: (
            <div
              css={css`
                display: flex;
                flex-direction: column;
                font-size: 12px;
                text-align: right;
              `}
            >
              {planet.resources.map((resource) => (
                <p
                  key={resource.resource_identifier}
                >{`${resource.resource_identifier.slice(3)} +${
                    resource.resource_richness_score
                  }`}</p>
              ))}
            </div>
          ),
        }))}
      />
    </div>
  )
}

export default PlanetsWithResources
