import React from 'react'
import H2 from 'src/components/H2'
import H4 from 'src/components/H4'
import { ModalBack, ModalContent } from 'src/components/Modal'
import P from 'src/components/P'
import { completeTaskBackgroundText, bonusBackgroundText } from 'src/styles/sharedStyles'
import styled, { css } from 'styled-components'

const Container = styled.div`
  display: flex;
  flex-direction: column;
  grid-row-gap: 15px;
  width: 320px;
`
const AboutQuest: React.FC = (): React.ReactElement => {

  return (
    <ModalContent css={css``}>
      <ModalBack></ModalBack>
        <>
          <Container>
            <div
              css={css`
                display: flex;
                flex-direction: column;
              `}
            >
              <H2
                css={css`
                  align-self: center;
                  ${completeTaskBackgroundText};
                `}
              >
                {'About Quests'}
              </H2>
            </div>
            <P>
              {'The primary purpose of resources is to complete quests. A quest consists of multiple objectives, all of which must be completed, in order, within a fixed amount of time. All xyz in the metaverse will be on the same quest at the same time with the same deadline.'}
            </P>
            <P>
              {'Each quest objective has a hidden resource cost, which will be revealed incrementally as the quest progresses, and a set of rewards, such as XP and bonus tokens. Objectives also have their own deadlines, and though they may be completed after the deadline, the resource cost will go up drastically.'}
            </P>
            <P>
              {'Completing every objective successfully before the quest expires will reward the xyz with an equal share of a $UST prize pool, split among all other xyz who completed the quest.'}
            </P>
            <P>
              {'Quests will be extremely challenging, with high resource requirements: only xyz with the most rich planets or the smartest marketplace dealings will be able to complete them.'}
            </P>
            <P css={css`color: lightcoral`}>
              {'Disclaimer: quests are in beta, use at your own risk. We are not responsible for any loss of funds.'}
            </P>
          </Container>
        </>
    </ModalContent>
  )
}

export default AboutQuest
