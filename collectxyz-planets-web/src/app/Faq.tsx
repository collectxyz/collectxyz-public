import React from 'react'
import BackgroundPage from 'src/components/BackgroundPage'
import H3 from 'src/components/H3'
import P from 'src/components/P'
import { css } from 'styled-components'

const Faq: React.FC = (): React.ReactElement => {
  return <BackgroundPage css={css`
    align-items: stretch;
    color: lightGrey;
    line-height: 20px;
  `}>
    <H3 css={css`
      margin-top: 20px; 
    `}>
      {'What is xyz?'}
    </H3>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'xyz is a collection of coordinates in three-dimensional space, represented as non-fungible tokens on the Terra blockchain. Each token is a tiny bundle of metadata: an xyz has an x, y, and z coordinate, and an ID. xyz is governed by a few laws along with the NFT standard: all sets of coordinates are unique; all token IDs are unique; and you can at any time relocate from one set of coordinates to another, while retaining the same ID, for a cost.'}
    </P>
    <H3 css={css`
      margin-top: 35px; 
    `}>
      {'What are planets?'}
    </H3>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'This website hosts the first metaverse application built on top of xyz: planets. In this metaverse, every set of coordinates in the xyz domain is home to a number of planets, waiting to be discovered. Once you own an xyz, you may begin exploring its coordinates and discovering those planets, each containing resources of varying rarity and richness.'}
    </P>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'This is just the start of a single metaverse built on xyz: more features are coming to the planets metaverse, and more metaverses are coming to xyz.'}
    </P>
    <H3 css={css`
      margin-top: 35px;
    `}>
      {'What happens when I transfer/sell/trade my xyz?'}
    </H3>
    <P css={css`
      margin-top: 10px;
    `}>
      {'When you transfer your xyz, you transfer the NFT itself and everything that is associated with it across all metaverses built on top of xyz. For example, this means you also transfer access to all of the planets at your coordinates to the new owner. Your xyz is the key to a multi-metaverse, and by giving away the key you give away everything it unlocks.'}
    </P>
    <H3 css={css`
      margin-top: 35px; 
    `}>
      {'What happens when I relocate my xyz to new coordinates?'}
    </H3>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'When you relocate your xyz, you keep the same ID on your NFT, but give up your old coordinates in exchange for new ones. This means that any data associated with your old coordinates is relinquished, any data associated with your ID is retained, and any data associated with your new coordinates is gained. For example, in the planets metaverse, all planets are associated with coordinates; therefore, if you relocate, you will lose access to the planets at your old coordinates, and gain access to any planets at the new ones.'}
    </P>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'This opens up endless decisions for metaverses built on top of xyz. For example, if all of the planets at your current coordinates have common resources with low richness, you can choose to leave behind your coordinates and explore new ones, or relocate to a known set of coordinates with better planets that somebody else left behind.'}
    </P>
    <H3 css={css`
      margin-top: 35px; 
    `}>
      {'How do I acquire xyz?'}
    </H3>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'The most direct way to acquire xyz is to mint on this website. There will be multiple minting periods, with potentially different pricing and mechanics, all of which will be announced on our '}
      <a href={'https://twitter.com/collectxyznft'} target={'_blank'} rel={'noreferrer'} css={css`
color: white;
text-decoration: underline;
      `}>
        {'Twitter'}
      </a>
      {' and Discord. Our initial minting period will consist of only 1000 xyz, available for 25 $UST each. Specifics with regards to timing will be released soon.'}
    </P>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'At launch, xyz will not immediately be on any NFT marketplaces: stay tuned for updates on this front. Being on a marketplace is of high priority for our team.'}
    </P>
    <H3 css={css`
      margin-top: 35px; 
    `}>
      {'What is on the roadmap for xyz?'}
    </H3>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'We plan on building in all directions. Specifically, this means:'}
    </P>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'- continuing development on the planets metaverse. The next feature is the ability to gather resources from the planets at your coordinates, which will be used as building blocks for more enhancements to your xyz. We would also love to improve the look/feel/art of the web application; we are not a team of artists, clearly, and much of the design/artwork was sourced or purchased externally, so we want make it a better experience.'}
    </P>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'- collaborating with existing projects. There are many exciting NFT projects in the right now, and we see many opportunities to work together to create a diverse and beautiful metaverse. Stay tuned for updates on this front: some things are already in motion. If you are an existing project and our philosophy sounds interesting to you, please reach out.'}
    </P>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'- creating a community of builders. Fundamentally, xyz is a primitive to build metaverses on, and then to build metaverses on those metaverses. The planets are just the beginning, just like Mirror was the first major application for Terra. And while we, like TFL, will continue to develop our own projects on top of xyz, we fundamentally believe that the real creativity will come from the community. Any and all builders welcome.'}
    </P>
    <H3 css={css`
      margin-top: 35px; 
    `}>
      {'I want to build on xyz. How can I get in touch?'}
    </H3>
    <P css={css`
      margin-top: 10px; 
    `}>
      {'If you want to build on xyz, collaborate with xyz, or speak with us for any reason, reach out to us on '}
      <a href={'https://twitter.com/collectxyznft'} target={'_blank'} rel={'noreferrer'} css={css`
color: white;
text-decoration: underline;
      `}>
        {'Twitter'}
      </a>
      {' or on our Discord. We look forward to hearing from you!'}
    </P>
  </BackgroundPage>
}

export default Faq
