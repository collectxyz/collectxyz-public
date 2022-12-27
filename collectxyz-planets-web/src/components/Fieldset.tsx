import React from 'react'
import { css } from 'styled-components'

interface FieldsetProps {
  className?: string
  legend?: string | React.ReactElement
  items: Array<{
    label: React.ReactElement
    value: React.ReactElement
  }>
}
const Fieldset: React.FC<FieldsetProps> = (props) => {
  return (
    <fieldset
      className={props.className}
      css={css`
        border: 1px ridge darkgrey;
        display: flex;
        flex-direction: column;
        color: lightGray;
        padding: 15px 10px;
        grid-row-gap: 10px;
      `}
    >
      {props.legend && (
        <legend
          css={css`
          margin-right: 20px;
          margin-left: 5px;
          padding-left: 5px;
          padding-right: 5px;
          font-size: 14px;
          white-space: nowrap;
        `}
        >
          {props.legend}
        </legend>
      )}
      <div
        css={css`
          margin-left: 5px;
          padding-left: 5px;
          margin-right: 5px;
          padding-right: 5px;
          display: flex;
          flex-direction: column;
          grid-gap: 10px;

          font-size: 14px;
        `}
      >
        {props.items.map((item, i) => (
          <div
            key={i}
            css={css`
            display: flex;
            justify-content: space-between;
            /* align-items: center; */
          `}
          >
            {item.label}
            {item.value}
          </div>
        ))}
      </div>
    </fieldset>
  )
}

export default Fieldset
