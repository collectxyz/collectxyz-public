import { Msg, TxInfo, TxLog } from '@terra-money/terra.js'
import { useConnectedWallet } from '@terra-money/wallet-provider'
import React, { useRef, useState } from 'react'
import { useQuery } from 'react-query'
import { Link } from 'react-router-dom'
import { Environment, useEnvironment } from 'src/contexts/environment.context'
import { useTerraClient } from 'src/hooks/useTerraClient'
import { ListingResponse } from 'src/models/listing.models'
import { PlanetResponse } from 'src/models/planet.models'
import { css } from 'styled-components'

interface Txs {
  next: number
  limit: number
  txs: TxInfo.Data[]
}

interface FeedItem {
  id: string
  timestamp: number
  description: React.ReactElement
}

const XyzLink: React.FC<{ id?: string }> = ({ id }) => {
  return (
    <>
      {id !== undefined && (
        <Link
          to={`/xyz/${id.split('#')[1]}`}
          css={css`
            text-decoration: underline;
            color: lightGray;
            &:hover {
              color: darkGray;
            }
          `}
        >
          {id}
        </Link>
      )}
    </>
  )
}

const getMessageDescription = (
  msg: Msg.Data,
  log: TxLog.Data,
  environment: Environment,
): React.ReactElement | undefined => {
  if (msg.type !== 'wasm/MsgExecuteContract') {
    return undefined
  }
  const wasmLogAttributes = log.events.find(
    (
      logEvent, // can probably replace with last element in events
    ) => logEvent.type === 'wasm',
  )?.attributes
  if (wasmLogAttributes === undefined) {
    return undefined
  }
  // const wasmLogAttributes = log.events[log.events.length - 1].attributes
  switch (msg.value.contract) {
    case environment.PLANETS_CONTRACT_ADDRESS: {
      if ((msg.value.execute_msg as Record<string, unknown>)['start_task']) {
        const xyzIdAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'xyz_id',
        )
        return (
          <p>
            <XyzLink id={xyzIdAttribute?.value}></XyzLink>
            {` launched a planetary exploration mission.`}
          </p>
        )
      }
      if ((msg.value.execute_msg as Record<string, unknown>)['complete_task']) {
        const xyzIdAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'xyz_id',
        )
        const planetAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'planet',
        )
        const planet: PlanetResponse | undefined =
          planetAttribute !== undefined
            ? JSON.parse(planetAttribute?.value)
            : undefined
        const resourceStrings =
          planet !== undefined
            ? planet.resources.map(
              (resource, i) =>
                ` ${resource.resource_identifier.slice(3)} +${
                  resource.resource_richness_score
                }`,
            )
            : []
        return planet !== undefined ? (
          <p>
            <XyzLink id={xyzIdAttribute?.value}></XyzLink>
            {` discovered a new planet, with ${resourceStrings}.`}
          </p>
        ) : (
          <p>
            <XyzLink id={xyzIdAttribute?.value}></XyzLink>
            {` discovered nothing but the endless void.`}
          </p>
        )
      }
      return undefined
    }
    case environment.RESOURCE_GATHERING_CONTRACT_ADDRESS: {
      if ((msg.value.execute_msg as Record<string, unknown>)['start_task']) {
        const xyzIdAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'xyz_id',
        )
        return (
          <p>
            <XyzLink id={xyzIdAttribute?.value}></XyzLink>
            {` dispatched a ship to gather resources.`}
          </p>
        )
      }
      if ((msg.value.execute_msg as Record<string, unknown>)['complete_task']) {
        const xyzIdAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'xyz_id',
        )
        const resourcesGatheredAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'resources_gathered',
        )
        const resourcesGathered: Record<string, string> | undefined =
          resourcesGatheredAttribute !== undefined
            ? JSON.parse(resourcesGatheredAttribute?.value)
            : undefined
        const resourceStrings =
          resourcesGathered !== undefined
            ? Object.entries(resourcesGathered).map(
              ([key, val], i) => ` ${parseInt(val) / 1000000} ${key.slice(3)}`,
            )
            : []
        return (
          <p>
            <XyzLink id={xyzIdAttribute?.value}></XyzLink>
            {` gathered${resourceStrings} from planets.`}
          </p>
        )
      }
      return undefined
    }
    case environment.MARKETPLACE_CONTRACT_ADDRESS: {
      if ((msg.value.execute_msg as Record<string, unknown>)['make_listing']) {
        const listingAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'listing',
        )
        const listing: ListingResponse | undefined =
          listingAttribute !== undefined
            ? JSON.parse(listingAttribute?.value)
            : undefined
        const price =
          listing !== undefined
            ? parseInt(listing.price_rmi) / 1000000
            : undefined
        const resources = listing?.resources.map((attribute, i) => {
          const amount = parseInt(attribute.amount) / 1000000
          const name = attribute.id.slice(3)
          return ` ${amount} ${name}`
        })
        return (
          <p>
            <XyzLink id={listing?.lister_xyz_id}></XyzLink>
            {` created a listing of${resources} for ${price} RMI.`}
          </p>
        )
      }
      if ((msg.value.execute_msg as Record<string, unknown>)['take_listing']) {
        const takerAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'taker_xyz_id',
        )
        const taker = takerAttribute?.value
        const listingAttribute = wasmLogAttributes.find(
          (attribute) => attribute.key === 'listing',
        )
        const listing: ListingResponse | undefined =
          listingAttribute !== undefined
            ? JSON.parse(listingAttribute?.value)
            : undefined
        const price =
          listing !== undefined
            ? parseInt(listing.price_rmi) / 1000000
            : undefined
        const resources = listing?.resources.map((attribute, i) => {
          const amount = parseInt(attribute.amount) / 1000000
          const name = attribute.id.slice(3)
          return ` ${amount} ${name}`
        })
        return (
          <p>
            <XyzLink id={taker}></XyzLink>
            {` purchased a listing of${resources} from `}
            <XyzLink id={listing?.lister_xyz_id}></XyzLink>
            {` for ${price} RMI.`}
          </p>
        )
      }
      return undefined
    }
    default: {
      return undefined
    }
  }
}

const useActivityFeed = () => {
  const { terraClient, api } = useTerraClient()
  const environmentContext = useEnvironment()
  const connectedWallet = useConnectedWallet()

  const [feedItems, setFeedItems] = useState<FeedItem[]>([])
  const seenFeedItems = useRef<Set<string>>(new Set([]))
  const startDate = useRef<number>(new Date().getTime())
  const query = async () => {
    const fcdUrl = (connectedWallet?.network as unknown as { fcd: string }).fcd || 'https://fcd.terra.dev'
    if (fcdUrl !== undefined) {
      const planetsResult: Txs = await (
        await fetch(
          `${fcdUrl}/v1/txs?offset=0&limit=10&account=${environmentContext.PLANETS_CONTRACT_ADDRESS}`,
        )
      ).json()
      const resourceGatheringResult: Txs = await (
        await fetch(
          `${fcdUrl}/v1/txs?offset=0&limit=10&account=${environmentContext.RESOURCE_GATHERING_CONTRACT_ADDRESS}`,
        )
      ).json()
      const marketplaceResult: Txs = await (
        await fetch(
          `${fcdUrl}/v1/txs?offset=0&limit=10&account=${environmentContext.MARKETPLACE_CONTRACT_ADDRESS}`,
        )
      ).json()
      const items = [
        ...planetsResult.txs,
        ...resourceGatheringResult.txs,
        ...marketplaceResult.txs,
      ]
        .map((tx) => {
          if (tx.code !== undefined) {
            return []
          }
          if (Date.parse(tx.timestamp) < startDate.current) {
            return []
          }
          return tx.tx.value.msg
            .map((msg, j): FeedItem | undefined => {
              const id = `${tx.txhash}_${j}`
              if (seenFeedItems.current.has(id)) {
                return undefined
              }
              seenFeedItems.current.add(id)
              const description =
                tx.logs !== undefined
                  ? getMessageDescription(msg, tx.logs[j], environmentContext)
                  : undefined
              return description !== undefined
                ? {
                  id,
                  timestamp: Date.parse(tx.timestamp),
                  description,
                }
                : undefined
            })
            .filter((item) => item !== undefined) as FeedItem[]
        })
        .flat()
      return [...items]
    }
    return []
  }
  useQuery<FeedItem[], unknown, FeedItem[]>(['activityFeed'], query, {
    enabled: api !== undefined,
    onSuccess: (data) => {
      setFeedItems((current) =>
        [...current, ...data].sort((a, b) => b.timestamp - a.timestamp),
      )
    },
    refetchInterval: 15000,
  })

  return feedItems
}

export default useActivityFeed
