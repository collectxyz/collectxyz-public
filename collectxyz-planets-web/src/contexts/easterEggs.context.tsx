import React, { createContext, useContext, useEffect, useState } from 'react'

interface EasterEggs {
  isBackgroundImageVisible: boolean
  setIsBackgroundImageVisible: (
    val: boolean | ((v: boolean) => boolean)
  ) => void
  isXyzMetadataVisible: boolean
  setIsXyzMetadataVisible: (val: boolean | ((v: boolean) => boolean)) => void
  isFiltersVisible: boolean
  setIsFiltersVisible: (val: boolean | ((v: boolean) => boolean)) => void
}
const EasterEggsContext = createContext<EasterEggs | undefined>(undefined)

export const EasterEggsContextProvider: React.FC = (props) => {
  const [isBackgroundImageVisible, setIsBackgroundImageVisible] = useState(
    JSON.parse(localStorage.getItem('isBackgroundImageVisible') || 'false'),
  )
  const [isXyzMetadataVisible, setIsXyzMetadataVisible] = useState(
    JSON.parse(localStorage.getItem('isXyzMetadataVisible') || 'true'),
  )
  const [isFiltersVisible, setIsFiltersVisible] = useState(
    JSON.parse(localStorage.getItem('isFiltersVisible') || 'true'),
  )

  useEffect(() => {
    localStorage.setItem(
      'isBackgroundImageVisible',
      JSON.stringify(isBackgroundImageVisible),
    )
  }, [isBackgroundImageVisible])
  useEffect(() => {
    localStorage.setItem(
      'isXyzMetadataVisible',
      JSON.stringify(isXyzMetadataVisible),
    )
  }, [isXyzMetadataVisible])
  useEffect(() => {
    localStorage.setItem('isFiltersVisible', JSON.stringify(isFiltersVisible))
  }, [isFiltersVisible])

  const easterEggs = {
    isBackgroundImageVisible,
    setIsBackgroundImageVisible,
    isXyzMetadataVisible,
    setIsXyzMetadataVisible,
    isFiltersVisible,
    setIsFiltersVisible,
  }
  return (
    <EasterEggsContext.Provider value={easterEggs}>
      {props.children}
    </EasterEggsContext.Provider>
  )
}

export const useEasterEggs = () => {
  const context = useContext(EasterEggsContext)
  if (context === undefined) {
    throw new Error('no EasterEggsContextProvider found')
  }
  return context
}
