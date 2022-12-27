import React from 'react'
import { Link } from 'react-router-dom'
import FadeInDelay from 'src/components/FadeInDelay'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalTypes } from 'src/components/Modal'
import XyzCard from 'src/components/XyzCard'
import useXyzTokens from 'src/hooks/useXyzTokens'
import { mediaDown } from 'src/styles/breakpoints'
import styled, { css } from 'styled-components'
const CollectionContainer = styled.div`
  display: flex;
  flex-direction: column;
  grid-gap: 30px;
  align-items: center;
  margin-top: 10px;
`
const Wrapping = styled.div`
  display: flex;
  flex-wrap: wrap;
  grid-gap: 40px;
  ${(props) => mediaDown('md')(`justify-content: center`)};
`
const LoadingCard = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
`

const Collection: React.FC = (): React.ReactElement => {
  const {
    result: { isLoading },
    limit,
    count,
    sortedTokens,
  } = useXyzTokens()

  return (
    <CollectionContainer>
      {sortedTokens !== undefined && count !== undefined && count > 0 && (
        <Wrapping>
          {sortedTokens.map((xyzResponse, i) => (
            <FadeInDelay
              key={xyzResponse.name}
              index={i}
              totalCount={Math.min(limit, sortedTokens.length)}
            >
              <XyzCard xyzResponse={xyzResponse} />
            </FadeInDelay>
          ))}
        </Wrapping>
      )}
      <LoadingCard>
        {!isLoading && !count && (
          <>
            <p
              css={css`
                color: lightGrey;
              `}
            >
              {'You do not own any xyz. '}
              <Link
                to={`?modal=${ModalTypes.Mint}`}
                css={css`
                  color: white;
                  text-decoration: underline;
                `}
              >
                {'Mint?'}
              </Link>
            </p>
          </>
        )}
        {isLoading && <LoadingIndicator speed={250}></LoadingIndicator>}
      </LoadingCard>
    </CollectionContainer>
  )
}

export default Collection
