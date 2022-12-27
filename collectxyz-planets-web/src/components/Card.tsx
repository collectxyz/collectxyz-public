import styled from 'styled-components'

const Card = styled.div`
  display: flex;
  flex-direction: column;
  width: 183px;
  box-shadow: 0 0 7px 3px lightgray;
  box-shadow: 0 0 4px 2px lightgray;
  &:hover {
    box-shadow: 0 0 4px 2px lightgray;
  }
  position: relative;
`
export default Card
