import React from 'react'
import styled from 'styled-components'

const Label = styled.label`
display: flex;
flex-direction: column;
align-self: stretch;
font-size: 14px;
color: lightGray;
`

const Input = styled.input`
border: 2px ridge white;
padding: 5px 5px;
align-self: stretch;
`
interface NumberInputProps {
  label: React.ReactElement | string
  value: number | undefined
  onChange: (val: number | undefined) => void
  placeholder: string
  onBlur?: () => void
}
const NumberInput: React.FC<NumberInputProps> = (props) => {
  const onChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    const v = Math.max(Math.min(parseInt(e.target.value), 1000), -1000)
    props.onChange(
      isNaN(v)
        ? undefined
        : v,
    )
  }

  return <Label>
    {props.label}
    <Input
      required
      placeholder={props.placeholder}
      type={'number'}
      inputMode={'numeric'}
      value={props.value !== undefined ? props.value : ''}
      onChange={onChange}
      onWheel={(e) => (e.target as any).blur()}
      onBlur={props.onBlur}
    />
  </Label>
}

export default NumberInput

