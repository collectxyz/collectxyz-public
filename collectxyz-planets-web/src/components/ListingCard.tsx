import React from 'react'
import { Link } from 'react-router-dom'
import Bg from 'src/assets/images/bg5.png'
import { ModalTypes } from 'src/components/Modal'
import useCountdown from 'src/hooks/useCountdown'
import { ListingResponse } from 'src/models/listing.models'
import { css } from 'styled-components'

interface ListingCardProps {
  listing: ListingResponse
  nameNumber: string
}
const ListingCard: React.FC<ListingCardProps> = (props) => {
  const { isComplete: isExpired, countdownView } = useCountdown(
    0,
    parseInt(props.listing.expired_at) / 1000000,
  )

  const containerCss = css`
  background-image: url('${Bg}');
    padding: 10px;
    display: flex;
    flex-direction: column;
    width: 143px;
    box-shadow: 0 0 4px 1px lightgray;
    transition: box-shadow 200ms ease-out;
    &:hover {
      box-shadow: 0 0 2px 1px lightgray;
    }
    position: relative;
    font-size: 12px;
    color: lightgray;
  `

  return (
    <div css={containerCss}>
      <Link
        to={`?modal=${ModalTypes.ListingDetail}&nameNumber=${props.nameNumber}&listingId=${props.listing.listing_id}`}
        css={css`
            position: absolute;
            top: 0px;
            left: 0px;
            bottom: 0px;
            right: 0px;
            z-index: 1;
          `}
      ></Link>
      <div
        css={css`
          background-color: transparent;
          position: relative;
          display: flex;
          border: 1px ridge rgba(255, 255, 255, 0.4);
          aspect-ratio: 1/1;
          flex-direction: column;
          grid-gap: 2px;
          padding: 5px;
        `}
      >
        {props.listing.resources.map((resource) => (
          <p
            key={resource.id}
            css={css`
            color: lightgray;
          `}
          >
            {`${resource.id.slice(3)} - ${parseInt(resource.amount) / 1000000}`}
          </p>
        ))}
      </div>
      <div
        css={css`
          display: flex;
          flex-direction: column;
        `}
      >
        <p
          css={css`
            margin-top: 5px;
            display: flex;
            align-items: center;
            max-height: 16px;
          `}
        >
          {`Price: ${parseInt(props.listing.price_rmi) / 1000000} RMI`}
        </p>
        <p
          css={css`
          color: ${isExpired ? 'darkgray' : 'lightgray'};
            /* margin-top: 4px; */
          `}
        >
          {isExpired ? 'Expired' : `Remaining: ${countdownView}`}
        </p>
        <p
          css={css`
            /* margin-top: 4px; */
            font-size: 12px;
          `}
        >
          {
            `${props.listing.lister_xyz_id}` === `xyz #${props.nameNumber}` ? `${props.listing.lister_xyz_id} (you)` : props.listing.lister_xyz_id
          }
        </p>
      </div>
    </div>
  )
}

export default ListingCard
