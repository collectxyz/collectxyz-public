import React, { createContext, useContext, useMemo } from 'react'

export interface Environment {
  CAPTCHA_PUBLIC_KEY: string
  CAPTCHA_URL: string
  XYZ_CONTRACT_ADDRESS: string
  PLANETS_CONTRACT_ADDRESS: string
  RANDOMNESS_CONTRACT_ADDRESS: string
  BONUS_TOKEN_CONTRACT_ADDRESS: string
  RESOURCE_GATHERING_CONTRACT_ADDRESS: string
  XYZ_ROCK_CONTRACT_ADDRESS: string
  XYZ_METAL_CONTRACT_ADDRESS: string
  XYZ_ICE_CONTRACT_ADDRESS: string
  XYZ_GAS_CONTRACT_ADDRESS: string
  XYZ_WATER_CONTRACT_ADDRESS: string
  XYZ_GEM_CONTRACT_ADDRESS: string
  XYZ_LIFE_CONTRACT_ADDRESS: string
  XYZ_XP_CONTRACT_ADDRESS: string
  MARKETPLACE_CONTRACT_ADDRESS: string
  QUEST_CONTRACT_ADDRESS: string
}
const EnvironmentContext = createContext<Environment | undefined>(undefined)

export const EnvironmentContextProvider: React.FC = (props) => {
  const environment = useMemo(() => ({
    CAPTCHA_PUBLIC_KEY: window.CAPTCHA_PUBLIC_KEY,
    CAPTCHA_URL: window.CAPTCHA_URL,
    XYZ_CONTRACT_ADDRESS: window.XYZ_CONTRACT_ADDRESS,
    PLANETS_CONTRACT_ADDRESS: window.PLANETS_CONTRACT_ADDRESS,
    RANDOMNESS_CONTRACT_ADDRESS: window.RANDOMNESS_CONTRACT_ADDRESS,
    BONUS_TOKEN_CONTRACT_ADDRESS: window.BONUS_TOKEN_CONTRACT_ADDRESS,
    RESOURCE_GATHERING_CONTRACT_ADDRESS: window.RESOURCE_GATHERING_CONTRACT_ADDRESS,
    XYZ_ROCK_CONTRACT_ADDRESS: window.XYZ_ROCK_CONTRACT_ADDRESS,
    XYZ_METAL_CONTRACT_ADDRESS: window.XYZ_METAL_CONTRACT_ADDRESS,
    XYZ_ICE_CONTRACT_ADDRESS: window.XYZ_ICE_CONTRACT_ADDRESS,
    XYZ_GAS_CONTRACT_ADDRESS: window.XYZ_GAS_CONTRACT_ADDRESS,
    XYZ_WATER_CONTRACT_ADDRESS: window.XYZ_WATER_CONTRACT_ADDRESS,
    XYZ_GEM_CONTRACT_ADDRESS: window.XYZ_GEM_CONTRACT_ADDRESS,
    XYZ_LIFE_CONTRACT_ADDRESS: window.XYZ_LIFE_CONTRACT_ADDRESS,
    XYZ_XP_CONTRACT_ADDRESS: window.XYZ_XP_CONTRACT_ADDRESS,
    MARKETPLACE_CONTRACT_ADDRESS: window.MARKETPLACE_CONTRACT_ADDRESS,
    QUEST_CONTRACT_ADDRESS: window.QUEST_CONTRACT_ADDRESS,
  }), [])

  return <EnvironmentContext.Provider value={environment}>
    {props.children}
  </EnvironmentContext.Provider>
}

export const useEnvironment = () => {
  const context = useContext(EnvironmentContext)
  if (context === undefined) {
    throw new Error('no EnvironmentContextProvider found')
  }
  return context
}
