import React from 'react'
import { css } from 'styled-components'

interface ResourceLabelProps {
  label: string
  imageSource: string
}

const ResourceLabel: React.FC<ResourceLabelProps> = (props) => {
  return (
    <div
      css={css`
        display: flex;
        align-items: center;
      `}
    >
      {/* <img
        src={props.imageSource}
        css={css`
        height: 14px;
        width: 14px;
        margin-right: 6px;
       `}
      ></img> */}
      {props.label}
    </div>
  )
}

export default ResourceLabel
