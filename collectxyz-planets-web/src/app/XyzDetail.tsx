import { ArrowDownward, ArrowUpward } from '@material-ui/icons'
import { useConnectedWallet } from '@terra-money/wallet-provider'
import React, { useState } from 'react'
import { matchPath, useLocation, useParams } from 'react-router'
import { Link } from 'react-router-dom'
import { TaskTypes } from 'src/app/StartTask'
import ExplorationTask from 'src/components/ExplorationTask'
import FilterPill from 'src/components/FilterPill'
import ListingCard from 'src/components/ListingCard'
import LoadingIndicator from 'src/components/LoadingIndicator'
import { ModalTypes } from 'src/components/Modal'
import ObjectiveCard from 'src/components/ObjectiveCard'
import PlanetsWithResources from 'src/components/PlanetsWithResources'
import ResourceGatheringTask from 'src/components/ResourceGatheringTask'
import Resources from 'src/components/Resources'
import { useEasterEggs } from 'src/contexts/easterEggs.context'
import { useEnvironment } from 'src/contexts/environment.context'
import useCountdown from 'src/hooks/useCountdown'
import useListings from 'src/hooks/useListings'
import useQuestCurrentConfig from 'src/hooks/useQuestCurrentConfig'
import useQuestGetCompleted from 'src/hooks/useQuestGetCompleted'
import useQuestGetObjectives from 'src/hooks/useQuestGetObjectives'
import useQuestGetReward from 'src/hooks/useQuestGetReward'
import useResourceBalance from 'src/hooks/useResourceBalance'
import useTerraFinderUrl from 'src/hooks/useTerraFinderUrl'
import useXyzNftInfo from 'src/hooks/useXyzNftInfo'
import useXyzPlanets from 'src/hooks/useXyzPlanets'
import useXyzPlanetTask from 'src/hooks/useXyzPlanetTask'
import useXyzResourceGatheringTask from 'src/hooks/useXyzResourceGatheringTask'
import { mediaDown } from 'src/styles/breakpoints'
import { completeTaskBackgroundText } from 'src/styles/sharedStyles'
import { walletTruncated } from 'src/utils'
import { css } from 'styled-components'

const buttonLink = css`
  color: darkGrey;
  ${completeTaskBackgroundText};
  padding: 2px 8px;
  border: 1px ridge darkGrey;
  &:hover {
    border: 1px ridge lightGrey;
  }
`

const navLink = (match: boolean) => css`
  text-decoration: none;
  color: ${match ? 'white' : 'darkgray'};
  font-size: 20px;
  padding: 10px 15px;
  border-bottom: ${match ? '1px solid white' : '1px solid transparent'};
  z-index: 1;
  white-space: nowrap;
  text-overflow: ellipsis;
`

const XyzDetail: React.FC = () => {
  const params = useParams<{ nameNumber: string }>()
  const nameNumber = params.nameNumber
  const name = `xyz #${nameNumber}`
  const location = useLocation()
  const connectedWallet = useConnectedWallet()

  const matchMarketplace = !!matchPath(
    location.pathname,
    '/xyz/:nameNumber/marketplace',
  )
  const matchQuest = !!matchPath(location.pathname, '/xyz/:nameNumber/quest')
  const matchOverview = !matchMarketplace && !matchQuest

  const { data, error, status, isSuccess, isFetched } = useXyzNftInfo(name)
  const isOwner = connectedWallet?.walletAddress === data?.owner

  const { result: planetsResult } = useXyzPlanets(data?.extension.coordinates)
  const {
    result: { data: planetTask, isFetched: isPlanetTaskFetched },
  } = useXyzPlanetTask(name)
  const {
    result: {
      data: resourceGatheringTask,
      isFetched: isResourceGatheringTaskFetched,
    },
  } = useXyzResourceGatheringTask(name)

  const isAllFetched =
    isFetched &&
    isPlanetTaskFetched &&
    isResourceGatheringTaskFetched &&
    planetsResult.isFetched

  const { isComplete: moveComplete, countdownView: moveCountdownView } = useCountdown(
    0,
    (data?.extension.arrival || 0) / 1000000,
  )

  const environmentContext = useEnvironment()
  const {
    result: { data: xyzXpBalanceData },
  } = useResourceBalance(name, environmentContext.XYZ_XP_CONTRACT_ADDRESS)

  const walletUrl = useTerraFinderUrl('address', data?.owner || '')

  const { isFiltersVisible, setIsFiltersVisible } = useEasterEggs()
  const [showAllListings, setShowAllListings] = useState(true)
  const [price, setPrice] = useState<string | undefined>()
  const [resource, setResource] = useState<string | undefined>()
  const [sortAscending, setSortAscending] = useState(true)
  const { result: listingsResult } = useListings({
    lister_xyz_id: showAllListings ? undefined : name,
    prices: price !== undefined ? [price] : undefined,
    resources: resource !== undefined ? [resource] : undefined,
    include_inactive: !showAllListings,
    ascending: sortAscending,
  })

  const { objectives, count: objectiveCount } = useQuestGetObjectives()
  const { quest } = useQuestCurrentConfig()
  const { completedObjectives, count: completedCount } = useQuestGetCompleted(name)
  const { reward } = useQuestGetReward(name)

  const {
    quest_name: questName,
    start_time: questStartTime,
    quest_duration_seconds: questDurationSeconds,
  } = quest || {}

  const { isComplete: questExpired, countdownView: questCountdownView } = useCountdown(
    parseInt(questStartTime || '') / 1000000, // nanoseconds
    parseInt(questDurationSeconds || '') * 1000, // seconds
  )

  return data !== undefined && isAllFetched ? (
    <>
      <div
        css={css`
          display: flex;
          align-items: center;
          margin-bottom: 10px;
          justify-content: space-between;
        `}
      >
        <div
          css={css`
            display: flex;
            align-items: center;
          `}
        >
          <p
            css={css`
              font-size: 24px;
              color: white;
            `}
          >
            {`${data.name}`}
          </p>
          <p
            css={css`
              font-size: 24px;
              margin-left: 20px;
            `}
          >
            {`[${data.extension.coordinates.x}, ${data.extension.coordinates.y}, ${data.extension.coordinates.z}]`}
          </p>
          {!moveComplete && (
            <p
              css={css`
                font-size: 16px;
                margin-left: 10px;
                color: darkgray;
              `}
            >
              {`(Relocating: ${moveCountdownView})`}
            </p>
          )}
        </div>
        {moveComplete && isOwner && (
          <Link
            to={`?modal=${ModalTypes.MoveCoordinates}&nameNumber=${
              data.name.split('#')[1]
            }&taskType=${TaskTypes.Exploration}`}
            css={css`
              ${buttonLink}
            `}
          >
            {'Relocate'}
          </Link>
        )}
      </div>
      <p
        css={css`
          font-size: 14px;
          color: lightgray;
          margin-bottom: 5px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        `}
      >
        {`Owner: `}
        <a
          href={walletUrl}
          target={'_blank'}
          rel={'noreferrer'}
          css={css`
            color: white;
            text-decoration: underline;
          `}
        >
          {walletTruncated(data.owner)}
        </a>
      </p>
      <p
        css={css`
          font-size: 14px;
          color: lightgray;
          margin-bottom: 20px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        `}
      >
        {`XP: `}
        {xyzXpBalanceData !== undefined && (
          <span>{parseInt(xyzXpBalanceData.balance) / 1000000}</span>
        )}
      </p>

      <div
        css={css`
          display: flex;
          align-items: center;
          overflow: auto;
        `}
      >
        <Link
          to={`/xyz/${data.name.split('#')[1]}/planets`}
          css={css`
            ${navLink(matchOverview)}
          `}
        >
          {'OVERVIEW'}
        </Link>
        <Link
          to={`/xyz/${data.name.split('#')[1]}/marketplace`}
          css={css`
            ${navLink(matchMarketplace)}
          `}
        >
          {'MARKETPLACE'}
        </Link>
        <Link
          to={`/xyz/${data.name.split('#')[1]}/quest`}
          css={css`
            ${navLink(matchQuest)}
          `}
        >
          {'QUEST (BETA)'}
        </Link>
      </div>

      <div
        css={css`
          height: 1px;
          background-color: darkgray;
          align-self: stretch;
          margin-bottom: 40px;
          opacity: 0.7;
        `}
      ></div>
      {matchOverview && (
        <div
          css={css`
            display: grid;
            grid-template-columns: 1fr 1.2fr 1.2fr;
            grid-gap: 45px;
            justify-content: space-between;
            width: 100%;
            ${mediaDown('md')`
          display: flex;
          flex-direction: column;
        `};
          `}
        >
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 25px;
            `}
          >
            <p
              css={css`
                margin-bottom: -2px;
                color: lightgray;
              `}
            >
              {'PLANETS'}
            </p>

            {(planetsResult.data === undefined ||
              planetsResult.data.length === 0) && (
              <div
                css={css`
                  display: flex;
                  flex-direction: column;
                  grid-gap: 20px;
                `}
              >
                <>
                  <p>{`[${data.extension.coordinates.x}, ${data.extension.coordinates.y}, ${data.extension.coordinates.z}] currently has no discovered planets.`}</p>
                </>
              </div>
            )}
            {planetsResult.data !== undefined &&
              planetsResult.data.length > 0 && (
              <PlanetsWithResources
                planets={planetsResult.data}
                xyzResponse={data}
              />
            )}
          </div>
          <div
            css={css`
              position: relative;
              display: flex;
              flex-direction: column;
              grid-gap: 25px;
            `}
          >
            <p
              css={css`
                margin-bottom: -2px;
                color: lightgray;
                font-size: 16px;
              `}
            >
              {'RESOURCES'}
            </p>
            <Resources name={name} />
          </div>
          <div
            css={css`
              display: flex;
              flex-direction: column;
              grid-gap: 20px;
              font-size: 14px;
              ${mediaDown('md')`
              `};
            `}
          >
            <p
              css={css`
                margin-bottom: 3px;
                color: lightgray;
                font-size: 16px;
              `}
            >
              {'TASKS'}
            </p>
            {planetTask !== undefined && (
              <ExplorationTask task={planetTask}></ExplorationTask>
            )}
            {resourceGatheringTask !== undefined && (
              <ResourceGatheringTask
                task={resourceGatheringTask}
              ></ResourceGatheringTask>
            )}
            {planetTask === undefined &&
              moveComplete &&
              planetsResult.data &&
              planetsResult.data?.length < 3 &&
              isOwner && (
              <Link
                to={`?modal=${ModalTypes.StartTask}&nameNumber=${
                  data.name.split('#')[1]
                }&taskType=${TaskTypes.Exploration}`}
                css={css`
                    ${buttonLink}
                    align-self: flex-start;
                  `}
              >
                {'Search for Planets'}
              </Link>
            )}
            {resourceGatheringTask === undefined && moveComplete && isOwner && (
              <Link
                to={`?modal=${ModalTypes.StartTask}&nameNumber=${
                  data.name.split('#')[1]
                }&taskType=${TaskTypes.ResourceGathering}`}
                css={css`
                  ${buttonLink}
                  align-self: flex-start;
                  opacity: ${planetsResult.data === undefined ||
                  planetsResult.data?.length === 0
                ? '0.7'
                : undefined};
                  pointer-events: ${planetsResult.data === undefined ||
                  planetsResult.data?.length === 0
                ? 'none'
                : undefined};
                `}
              >
                {'Gather Resources'}
              </Link>
            )}
          </div>
        </div>
      )}
      {matchMarketplace && (
        <div
          css={css`
            display: flex;
            justify-content: space-between;
            width: 100%;
            grid-gap: 24px;
            ${mediaDown('md')`
          display: flex;
          flex-direction: column;
        `};
          `}
        >
          <div
            css={css`
              position: relative;
              display: flex;
              flex-direction: column;
              grid-gap: 15px;
              flex-grow: 2;
            `}
          >
            <div
              css={css`
                position: relative;
                display: flex;
                flex-direction: column;
                grid-gap: 10px;
                ${mediaDown('md')`
          grid-gap: 15px;
        `};
              `}
            >
              <div
                css={css`
                  display: flex;
                  justify-content: space-between;
                `}
              >
                <div
                  css={css`
                    display: flex;
                    align-items: center;
                    grid-gap: 20px;
                  `}
                >
                  <p
                    css={css`
                      font-size: 12px;
                      cursor: pointer;
                      ${completeTaskBackgroundText}
                      display: flex;
                      align-items: center;
                    `}
                    role={'button'}
                    onClick={(): void => {
                      setSortAscending((v) => !v)
                    }}
                  >
                    {sortAscending ? 'Sort Descending' : 'Sort Ascending'}
                    {sortAscending ? (
                      <ArrowDownward
                        css={css`
                          transform: scale(0.5);
                        `}
                      ></ArrowDownward>
                    ) : (
                      <ArrowUpward
                        css={css`
                          transform: scale(0.5);
                        `}
                      ></ArrowUpward>
                    )}
                  </p>
                  <p
                    css={css`
                      font-size: 12px;
                      cursor: pointer;
                      ${completeTaskBackgroundText}
                    `}
                    role={'button'}
                    onClick={(): void => {
                      setIsFiltersVisible((v) => !v)
                    }}
                  >
                    {isFiltersVisible ? 'Hide Filters' : 'Show Filters'}
                  </p>
                </div>

                <Link
                  to={`?modal=${ModalTypes.MakeListing}&nameNumber=${
                    data.name.split('#')[1]
                  }`}
                  css={css`
                    ${buttonLink}
                  `}
                >
                  {'Create Listing'}
                </Link>
              </div>
              {isFiltersVisible && (
                <>
                  <div
                    css={css`
                      position: relative;
                      display: flex;
                      flex-wrap: wrap;
                      grid-gap: 10px;
                    `}
                  >
                    <FilterPill
                      onClick={() => {
                        setShowAllListings(true)
                      }}
                      selected={showAllListings === true}
                    >
                      {'ALL ACTIVE LISTINGS'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setShowAllListings(false)
                      }}
                      selected={showAllListings === false}
                    >
                      {'YOUR LISTINGS'}
                    </FilterPill>
                  </div>
                  <div
                    css={css`
                      position: relative;
                      display: flex;
                      flex-wrap: wrap;
                      grid-gap: 10px;
                    `}
                  >
                    <FilterPill
                      onClick={() => {
                        setPrice(undefined)
                      }}
                      selected={price === undefined}
                    >
                      {'ANY PRICE'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setPrice('5000000')
                      }}
                      selected={price === '5000000'}
                    >
                      {'5 RMI'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setPrice('10000000')
                      }}
                      selected={price === '10000000'}
                    >
                      {'10 RMI'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setPrice('50000000')
                      }}
                      selected={price === '50000000'}
                    >
                      {'50 RMI'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setPrice('100000000')
                      }}
                      selected={price === '100000000'}
                    >
                      {'100 RMI'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setPrice('500000000')
                      }}
                      selected={price === '500000000'}
                    >
                      {'500 RMI'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setPrice('1000000000')
                      }}
                      selected={price === '1000000000'}
                    >
                      {'1000 RMI'}
                    </FilterPill>
                  </div>
                  <div
                    css={css`
                      position: relative;
                      display: flex;
                      flex-wrap: wrap;
                      grid-gap: 10px;
                    `}
                  >
                    <FilterPill
                      onClick={() => {
                        setResource(undefined)
                      }}
                      selected={resource === undefined}
                    >
                      {'ANY RESOURCES'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzROCK')
                      }}
                      selected={resource === 'xyzROCK'}
                    >
                      {'ROCK'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzMETAL')
                      }}
                      selected={resource === 'xyzMETAL'}
                    >
                      {'METAL'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzICE')
                      }}
                      selected={resource === 'xyzICE'}
                    >
                      {'ICE'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzGAS')
                      }}
                      selected={resource === 'xyzGAS'}
                    >
                      {'GAS'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzWATER')
                      }}
                      selected={resource === 'xyzWATER'}
                    >
                      {'WATER'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzGEM')
                      }}
                      selected={resource === 'xyzGEM'}
                    >
                      {'GEM'}
                    </FilterPill>
                    <FilterPill
                      onClick={() => {
                        setResource('xyzLIFE')
                      }}
                      selected={resource === 'xyzLIFE'}
                    >
                      {'LIFE'}
                    </FilterPill>
                  </div>
                </>
              )}
            </div>

            <div
              css={css`
                height: 1px;
                background-color: darkgray;
                align-self: stretch;
                opacity: 0.7;
              `}
            ></div>

            <div
              css={css`
                position: relative;
                display: flex;
                flex-wrap: wrap;
                grid-gap: 23px;
                ${mediaDown('sm')`
                  flex-direction: column;
                  align-items: center;
                `};
              `}
            >
              {listingsResult.data !== undefined &&
                listingsResult.data.pages.map((page) =>
                  page.listings.map((listing) => (
                    <ListingCard
                      key={listing.listing_id}
                      listing={listing}
                      nameNumber={nameNumber}
                    />
                  )),
                )}
              {listingsResult.data !== undefined &&
                listingsResult.data.pages[0].listings.length === 0 && (
                <p
                  css={css`
                    color: lightgray;
                  `}
                >
                  {'No listings found!'}
                </p>
              )}
            </div>
            {listingsResult.isFetching && <LoadingIndicator />}

            {listingsResult.hasNextPage && !listingsResult.isFetching && (
              <p
                css={css`
                  font-size: 12px;
                  cursor: pointer;
                  ${completeTaskBackgroundText}
                `}
                role={'button'}
                onClick={(): void => {
                  listingsResult.fetchNextPage()
                }}
              >
                {'Load more'}
              </p>
            )}
          </div>
          <div
            css={css`
              position: relative;
              display: flex;
              flex-direction: column;
              grid-gap: 25px;
              width: 230px;
              flex-shrink: 0;
              ${mediaDown('md')`
                display: none;
              `};
            `}
          >
            <Resources name={name} />
          </div>
        </div>
      )}
      {matchQuest && (
        <div
          css={css`
            display: flex;
            justify-content: space-between;
            width: 100%;
            grid-gap: 30px;
            ${mediaDown('md')`
              display: flex;
              flex-direction: column;
            `};
          `}
        >
          <div
            css={css`
              display: flex;
              flex-direction: column;
            `}
          >
            <div
              css={css`
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                margin-bottom: 20px;
              `}
            >
              <div
                css={css`
                  display: flex;
                  flex-direction: column;
                `}
              >
                <p
                  css={css`
                    color: white;
                    margin-bottom: 10px;
                    font-size: 25px;
                    display: flex;
                    align-items: center;
                  `}
                >
                  {`${questName}`}
                </p>
                <div
                  css={css`
                    display: flex;
                    align-items: center;
                    margin-bottom: 5px;
                  `}
                >
                  <div
                    css={css`
                      border: 1px solid white;
                      height: 14px;
                      width: 100px;
                      margin-right: 10px;
                      position: relative;
                    `}
                  >
                    <div
                      css={css`
                        position: absolute;
                        top: 0;
                        bottom: 0;
                        left: 0;
                        width: ${(completedCount / objectiveCount) * 100}px;
                        background-color: lightGray;
                      `}
                    ></div>
                  </div>
                  <span
                    css={css`
                      font-size: 14px;
                      color: lightGray;
                    `}
                  >
                    {`(${completedCount}/${objectiveCount} objectives complete)`}
                  </span>
                </div>
              </div>
              <div
                css={css`
                  display: flex;
                  flex-direction: column;
                `}
              >
                <div
                  css={css`
                    margin-bottom: 15px;
                  `}
                >
                  <Link
                    to={`?modal=${ModalTypes.CompleteQuest}&nameNumber=${nameNumber}`}
                    css={css`
                      ${buttonLink}
                    `}
                  >
                    {'Complete Quest'}
                  </Link>
                </div>
                <p
                  css={css`
                    color: lightGray;
                    font-size: 14px;
                    text-align: right;
                  `}
                >
                  { questExpired
                    ? 'this quest has ended.' : (
                      <>
                        {'Remaining: '}
                        <span css={css``}>{questCountdownView}</span>
                      </>
                    )}

                </p>
              </div>
            </div>
            <div
              css={css`
                position: relative;
                display: flex;
                flex-wrap: wrap;
                grid-gap: 35px;
                ${mediaDown('sm')`
                  flex-direction: column;
                  align-items: center;
                `};
              `}
            >
              {objectives !== undefined &&
                objectives.map((objective) => {
                  const temp = completedObjectives?.filter((c) => c.objective.objective_id === objective.objective_id)
                  const completedObjective = temp && temp.length > 0 ? temp[0] : null

                  return <ObjectiveCard nameNumber={nameNumber} objective={objective} completedObjective={completedObjective} />
                })}
            </div>
          </div>
          <div>
            <Link
              to={`?modal=${ModalTypes.AboutQuest}`}
              css={css`
                text-decoration: underline;
                white-space: nowrap;
                text-overflow: ellipsis;
                cursor: pointer;
                color: white;
                &:hover {
                  color: lightGrey;
                }
              `}
            >
              about quests.
            </Link>
            <div
              css={css`
                position: relative;
                display: flex;
                flex-direction: column;
                grid-gap: 25px;
                width: 230px;
                margin-top: 12px;
                flex-shrink: 0;
                ${mediaDown('md')`
                  display: none;
                `};
              `}
            >
              <Resources name={name} />
            </div>
          </div>
        </div>
      )}
    </>
  ) : isAllFetched ? null : (
    <LoadingIndicator
      css={css`
        align-self: center;
      `}
    />
  )
}

export default XyzDetail
