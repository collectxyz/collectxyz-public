import React from 'react'
import Rock from 'src/assets/images/Rock.png'
import Fieldset from 'src/components/Fieldset'
import ResourceLabel from 'src/components/ResourceLabel'
import { useEnvironment } from 'src/contexts/environment.context'
import useResourceBalance from 'src/hooks/useResourceBalance'
import { css } from 'styled-components'

interface ResourcesProps {
  name: string
  className?: string
}
const Resources: React.FC<ResourcesProps> = (props) => {
  const environmentContext = useEnvironment()

  const {
    result: { data: xyzRockBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_ROCK_CONTRACT_ADDRESS)
  const {
    result: { data: xyzMetalBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_METAL_CONTRACT_ADDRESS)
  const {
    result: { data: xyzIceBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_ICE_CONTRACT_ADDRESS)
  const {
    result: { data: xyzGasBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_GAS_CONTRACT_ADDRESS)
  const {
    result: { data: xyzWaterBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_WATER_CONTRACT_ADDRESS)
  const {
    result: { data: xyzGemBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_GEM_CONTRACT_ADDRESS)
  const {
    result: { data: xyzLifeBalanceData },
  } = useResourceBalance(props.name, environmentContext.XYZ_LIFE_CONTRACT_ADDRESS)

  const resourceItems = [
    ...(xyzRockBalanceData !== undefined &&
    parseInt(xyzRockBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock} label={'ROCK'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzRockBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
    ...(xyzMetalBalanceData !== undefined &&
    parseInt(xyzMetalBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock}label={'METAL'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzMetalBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
    ...(xyzIceBalanceData !== undefined &&
    parseInt(xyzIceBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock}label={'ICE'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzIceBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
    ...(xyzGasBalanceData !== undefined &&
    parseInt(xyzGasBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock}label={'GAS'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzGasBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
    ...(xyzWaterBalanceData !== undefined &&
    parseInt(xyzWaterBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock}label={'WATER'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzWaterBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
    ...(xyzGemBalanceData !== undefined &&
    parseInt(xyzGemBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock}label={'GEM'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzGemBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
    ...(xyzLifeBalanceData !== undefined &&
    parseInt(xyzLifeBalanceData.balance) > 0
      ? [
        {
          label: <ResourceLabel imageSource={Rock}label={'LIFE'}></ResourceLabel>,
          value: (
            <p>{parseInt(xyzLifeBalanceData.balance) / 1000000}</p>
          ),
        },
      ]
      : []),
  ]

  return resourceItems.length > 0
    ? <Fieldset
      items={resourceItems}
      className={props.className}
    ></Fieldset>
    : <p css={css``}>{'This xyz currently has no gathered resources.'}</p>
}

export default Resources
