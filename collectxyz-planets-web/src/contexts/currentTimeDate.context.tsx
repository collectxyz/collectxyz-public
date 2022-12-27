import React, { createContext, useContext, useEffect, useRef, useState } from 'react'

interface CurrentTimeDate {
  currentTimeDate: Date
}
const CurrentTimeDateContext = createContext<CurrentTimeDate | undefined>(undefined)

export const CurrentTimeDateContextProvider: React.FC = (props) => {
  const [currentTimeDate, setCurrentTimeDate] = useState(new Date())
  const intervalRef = useRef<NodeJS.Timer | undefined>()
  useEffect(() => {
    intervalRef.current = setInterval(() => {
      setCurrentTimeDate(new Date())
    }, 1000)
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [])

  return <CurrentTimeDateContext.Provider value={{currentTimeDate}}>
    {props.children}
  </CurrentTimeDateContext.Provider>
}

export const useCurrentTimeDate = () => {
  const context = useContext(CurrentTimeDateContext)
  if (context === undefined) {
    throw new Error('no CurrentTimeDateContextProvider found')
  }
  return context
}
