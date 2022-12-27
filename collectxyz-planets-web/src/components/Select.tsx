import React from 'react'
import styled from 'styled-components'

const Label = styled.label`
display: flex;
flex-direction: column;
align-self: stretch;
font-size: 14px;
color: lightGray;
`

const SelectContainer = styled.select`
border: 2px ridge white;

`

const Option = styled.option`
padding: 5px 5px;
`
interface SelectProps {
  label: React.ReactElement | string
  value: string | undefined
  options: Array<{
    value?: string
    display: string
  }>
  onChange: (val: string | undefined) => void
  className?: string
}
const Select: React.FC<SelectProps> = (props) => {
  const changeHandler = (e: React.ChangeEvent<HTMLSelectElement>): void => {
    props.onChange(e.target.value)
  }

  return <Label>
    {props.label}
    <SelectContainer onChange={changeHandler} value={props.value} className={props.className}>
      {props.options.map((option, i) => (
        <Option value={option.value} key={`${option.value}${i}`} disabled={!option.value}>
          {option.display}
        </Option>
      ))}
    </SelectContainer>
  </Label>
}

export default Select

