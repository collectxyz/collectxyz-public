export const breakpoints = {
  sm: 320,
  md: 769,
  lg: 1025,
  xl: 1200,
  xxl: 1500,
}

export const mediaDown = (key: keyof typeof breakpoints) => {
  return (style: TemplateStringsArray | string) =>
    `@media (max-width: ${breakpoints[key]}px) { ${style} }`
}
