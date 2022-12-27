import React from 'react'
import { css, keyframes } from 'styled-components'

interface FadeInDelayProps {
  index: number
  totalCount: number
}

const cardWait = keyframes`
  from {
opacity: 0;
  }
  to {
opacity: 0;
  }
`
const cardFadeIn = keyframes`
  from {
opacity: 0;
  }
  to {
opacity: 1;
  }
`

const FadeInDelay: React.FC<FadeInDelayProps> = (props) => {
  return (
    <div
      css={css`
        animation: ${cardWait}
            ${((props.index % props.totalCount) * 0.25) / props.totalCount}s
            ease-out,
          ${cardFadeIn} 1.0s ease-out
            ${((props.index % props.totalCount) * 0.25) / props.totalCount}s;
      `}
    >
      {props.children}
    </div>
  )
}

export default FadeInDelay
