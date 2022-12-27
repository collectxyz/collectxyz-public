import React from 'react'
import Bonus from 'src/assets/images/bonus.png'
import { BonusTokenBalance } from 'src/hooks/useBonusTokenBalance'
import { css } from 'styled-components'

interface BonusTokenCountProps {
  bonusTokenBalance: BonusTokenBalance
  className?: string
}
const BonusTokenCount: React.FC<BonusTokenCountProps> = (props) => {
  return (
    <div
      className={props.className}
      css={css`
        display: flex;
        align-items: center;
        grid-gap: 10px;
      `}
    >
      <img
        src={Bonus}
        css={css`
          width: 20px;
          height: 20px;
        `}
      ></img>
      <p
        css={css`
          color: lightgray;
        `}
      >
        {parseInt(props.bonusTokenBalance.balance) / 1000000}
      </p>
    </div>
  )
}

export default BonusTokenCount
