import React from 'react'
import styled from 'styled-components'
import { variant } from 'styled-system'

export type ColorVariants = 'primary' | 'secondary' | 'tertiary'
const colorvariantPropName = 'colorvariant'
const colorvariants = variant<Record<string, unknown>, ColorVariants, typeof colorvariantPropName>({
  prop: colorvariantPropName,
  variants: {
    primary: {
      border: '3px ridge lightgrey',
      backgroundColor: 'white',
      color: 'blue',

      '&:hover': {
        'color': 'red',
        'border': '3px solid transparent',
        'border-image': 'linear-gradient(to bottom right, #b827fc 0%, #2c90fc 25%, #b8fd33 50%, #fec837 75%, #fd1892 50%)',
        'border-image-slice': '1',
      },
    },
    secondary: {
      border: '3px ridge white',
      backgroundColor: 'royalblue',
      color: 'white',
      transition: 'border 150ms ease-out, background-color 150ms ease-out',
      '&:hover': {
        border: '3px ridge lightgrey',
        backgroundColor: 'blue',
      },

      // '&:hover': {
      //   'color': 'red',
      //   'border': '3px solid transparent',
      //   'backgroundColor': 'white',
      //   'border-image': 'linear-gradient(to bottom right, #b827fc 0%, #2c90fc 25%, #b8fd33 50%, #fec837 75%, #fd1892 50%)',
      //   'border-image-slice': '1',
      // },
      '&:disabled': {
        'color': 'white',
        'border': '3px solid transparent',
        'backgroundColor': 'gray',
      },
    },

    tertiary: {
      border: '2px ridge transparent',
      backgroundColor: 'transparent',
      color: 'white',

      'border-image': 'linear-gradient(to bottom right, #b827fc 0%, #2c90fc 25%, #b8fd33 50%, #fec837 75%, #fd1892 50%)',
      'border-image-slice': '1',
    },
  },
})

type SizeVariants = 'smallest' | 'small' | 'large'
const sizevariantPropName = 'sizevariant'
const sizevariants = variant<Record<string, unknown>, SizeVariants, typeof sizevariantPropName>({
  prop: sizevariantPropName,
  variants: {
    smallest: {
      padding: ['0px 0px'],
      fontSize: ['14px', '14px', '18px'],
    },
    small: {
      padding: ['2px 12px'],
      fontSize: '14px',
    },
    large: {
      padding: ['10px 15px'],
      fontSize: '20px',
    },
  },
})

const Button = styled.button<ButtonProps & React.HTMLProps<HTMLButtonElement>>`
  cursor: pointer;
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: center;
  text-decoration: underline;
  text-transform: uppercase;

  &:disabled {
    cursor: default;
  }
  ${colorvariants}
  ${sizevariants}
`

interface ButtonProps {
  [colorvariantPropName]: ColorVariants
  [sizevariantPropName]: SizeVariants
}

export default Button
