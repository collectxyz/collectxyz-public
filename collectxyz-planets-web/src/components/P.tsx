import { fonts } from 'src/styles/constants'
import styled from 'styled-components'
import { color, ColorProps, grid, GridProps, layout, LayoutProps, position, PositionProps, space, SpaceProps, typography, TypographyProps } from 'styled-system'

const P = styled.p<ColorProps & SpaceProps & LayoutProps & GridProps & PositionProps & TypographyProps>`
  font-family: ${fonts.primaryRegular};
  font-size: 14px;
  ${space}
  ${color}
  ${layout}
  ${grid}
  ${position}
  ${typography}
`

export default P
