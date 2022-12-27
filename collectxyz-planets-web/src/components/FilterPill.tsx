import styled from 'styled-components'

const FilterPill = styled.button<{ selected?: boolean }>`
  color: ${(props) => (props.selected ? 'white' : 'darkgray')};
  padding: 2px 8px;
  border: 1px ridge ${(props) => (props.selected ? 'white' : 'darkgray')};
  opacity: ${(props) => (props.selected ? undefined : '0.9')};
  box-shadow: ${(props) =>
    props.selected ? '0 0 3px 1px lightgray' : undefined};
  border-radius: 4px;
  background-color: transparent;
  font-size: 12px;
  cursor: pointer;
  &:hover {
    border: 1px ridge lightGrey;
    opacity: 1;
  }
`

export default FilterPill
