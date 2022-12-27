import { mediaDown } from 'src/styles/breakpoints'
import styled from 'styled-components'

const BackgroundPage = styled.div`
  display: flex;
  flex-direction: column;
  color: white;
  flex-grow: 2;
  padding-bottom: 80px;
  width: 900px;
  ${(props) => mediaDown('lg')(`width: 100%`)};
  ${(props) => mediaDown('lg')(`padding: 0px 10px 40px`)};
`

export default BackgroundPage
