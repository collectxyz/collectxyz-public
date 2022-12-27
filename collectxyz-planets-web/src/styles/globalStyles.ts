import { createGlobalStyle } from 'styled-components'
import PressStart2PRegular from 'src/assets/fonts/PressStart2P-Regular.ttf'
import TimesPixelated from 'src/assets/fonts/TimesPixelated.woff'
import { fonts } from 'src/styles/constants'

const GlobalStyle = createGlobalStyle`
  @font-face {
    font-family: ${fonts.highlight};
    src: url('${PressStart2PRegular}') format('truetype');
  }

  @font-face {
    font-family: ${fonts.pixelated};
    src: url('${TimesPixelated}') format('truetype');
  }

  * {
    text-decoration: none;
    font-weight: normal;
    font-family: ${fonts.primaryRegular}, serif,sans-serif;
    box-sizing: border-box;
    padding: 0;
    margin: 0;
  }
  a:hover {
    color: lightGrey;
  }
  button {
    border: none;
  }
`

export default GlobalStyle
