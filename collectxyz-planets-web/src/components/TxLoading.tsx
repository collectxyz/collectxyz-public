import { TxInfo } from '@terra-money/terra.js'
import React from 'react'
import Loading from 'src/components/Loading'
import P from 'src/components/P'
import useTerraFinderUrl from 'src/hooks/useTerraFinderUrl'
import styled, { css } from 'styled-components'

const TxContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 304px;
  grid-row-gap: 20px;
`

interface TxLoadingProps {
  txHash: string
  tx?: TxInfo
  successElement?: React.ReactElement
}
const TxLoading: React.FC<TxLoadingProps> = ({ tx, txHash, ...props }) => {
  const txUrl = useTerraFinderUrl('tx', txHash)

  return (
    <TxContainer css={css``}>
      {tx === undefined && (
        <Loading
          color={'lightGrey'}
          isLoading={tx === undefined}
        >
          <P
            css={css`
          font-size: 20px;
          `}
          >
            {tx === undefined && 'BROADCASTING'}
          </P>
        </Loading>
      )}
      {tx?.code !== undefined && (
        <>
          <P
            css={css`
            color: red;
            text-align: center;
            font-size: 20px;
          `}
          >
            {'FAILURE'}
          </P>
          <P
            css={css`
            color: red;
            text-align: center;
          `}
          >
            {tx.raw_log}
          </P>
        </>
      )}
      {tx !== undefined && tx.code === undefined && (
        props.successElement
      )}
      <a
        href={txUrl}
        target={'_blank'}
        rel={'noreferrer'}
        css={css`
          color: white;
          text-decoration: underline;
        `}
      >
        {'View transaction on Terra Finder'}
      </a>
    </TxContainer>
  )
}

export default TxLoading
