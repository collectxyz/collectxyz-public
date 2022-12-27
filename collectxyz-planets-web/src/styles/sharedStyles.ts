import { css, keyframes } from 'styled-components'

const bgAnimation = keyframes`
  from {
    background-position: 0% 0%;
  }
  to {
    background-position: 100% 0%;
  }
`
export const bonusBackgroundText = css`
  background-image: linear-gradient(
    to right,
    #fd1892,
    #b827fc,
    #2c90fc,
    #fd1892,
    #b827fc,
    #2c90fc,
    #fd1892,
    #b827fc,
    #2c90fc
  );
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: ${bgAnimation} 3.5s linear infinite;
  background-size: 400% 100%;
`

export const completeTaskBackgroundText = css`
  background-image: linear-gradient(
    to right,
    white,
    lightgray,
    darkgray,
    white,
    lightgray,
    darkgray,
    white,
    lightgray,
    darkgray
  );
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: ${bgAnimation} 3.5s linear infinite;
  background-size: 400% 100%;
`
