import React from 'react'
import { Link } from 'react-router-dom'
import BonusTokenCount from 'src/components/BonusTokenCount'
import { ModalTypes } from 'src/components/Modal'
import useBonusTokenBalance from 'src/hooks/useBonusTokenBalance'
import useBonusTokenCountdown from 'src/hooks/useBonusTokenCountdown'
import { mediaDown } from 'src/styles/breakpoints'
import { bonusBackgroundText } from 'src/styles/sharedStyles'
import { css } from 'styled-components'

const BonusBox = () => {
  const { renderCountdown, countdownView } = useBonusTokenCountdown()
  const { result: bonusResult } = useBonusTokenBalance()

  return (
    <div
      css={css`
        position: fixed;
        top: 30px;
        right: 20px;
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        ${(props) =>
      mediaDown('xl')(`
        position: relative;
        top: auto;
        right: auto;
        margin-bottom: 10px;
        `)};
      `}
    >
      {renderCountdown && (
        <Link
          to={`?modal=${ModalTypes.ClaimBonus}`}
          css={css`
            color: darkGrey;
            font-size: 12px;
            height: 30px;
          `}
        >
          {`Next bonus: ${countdownView}`}
        </Link>
      )}
      {!renderCountdown && (
        <Link
          to={`?modal=${ModalTypes.ClaimBonus}`}
          css={css`
            ${bonusBackgroundText};
            font-size: 16px;
            height: 30px;
          `}
        >
          {'CLAIM BONUS.'}
        </Link>
      )}
      {bonusResult.data && parseInt(bonusResult.data.balance) > 0 && (
        <BonusTokenCount
          bonusTokenBalance={bonusResult.data}
          css={css`
            ${(props) =>
          mediaDown('xl')(`
        display: none;
        `)};
          `}
        />
      )}
    </div>
  )
}

export default BonusBox
