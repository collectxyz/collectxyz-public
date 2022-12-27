import 'core-js/stable'
import dotenv from 'dotenv'
import Express from 'express'
import fs from 'fs'
import 'regenerator-runtime/runtime'

const buildManifestJSON: Record<string, string> = JSON.parse(fs.readFileSync('public/build-manifest.json').toString())

const app = Express()

const listen = async (): Promise<void> => {
  dotenv.config()
  app.use('/public', Express.static('public'))

  app.use(async (req, res) => {
    res.set('X-XSS-Protection', '1; mode=block')
    res.set('Strict-Transport-Security', 'max-age=31536000; includeSubDomains')
    res.set('X-Frame-Options', 'DENY')
    res.set('Cache-Control', 'no-cache, no-store, max-age=0, must-revalidate')

    // render and send
    const html = `
    <!doctype html>
      <html lang='en-us'>
        <head>
          <title>xyz</title>
          <meta name="description" content="Explore the metaverse, starting with xyz."/>
          <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1">   
          <meta property="og:title" content="xyz" />
          <meta property="og:description" content="Explore the metaverse, starting with xyz" />
          <meta property="og:type" content="website" />
          <meta property="og:url" content="https://app.collectxyz.com" />
          <meta property="og:image" content="https://cdn.collectxyz.com/logo.png" />
          <link rel='shortcut icon' href='https://cdn.collectxyz.com/favicon.ico' />
          <script>window.CAPTCHA_PUBLIC_KEY='${process.env.CAPTCHA_PUBLIC_KEY}'</script>
          <script>window.CAPTCHA_URL='${process.env.CAPTCHA_URL}'</script>
          <script>window.XYZ_CONTRACT_ADDRESS='${process.env.XYZ_CONTRACT_ADDRESS}'</script>
          <script>window.PLANETS_CONTRACT_ADDRESS='${process.env.PLANETS_CONTRACT_ADDRESS}'</script>
          <script>window.RANDOMNESS_CONTRACT_ADDRESS='${process.env.RANDOMNESS_CONTRACT_ADDRESS}'</script>
          <script>window.BONUS_TOKEN_CONTRACT_ADDRESS='${process.env.BONUS_TOKEN_CONTRACT_ADDRESS}'</script>
          <script>window.RESOURCE_GATHERING_CONTRACT_ADDRESS='${process.env.RESOURCE_GATHERING_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_ROCK_CONTRACT_ADDRESS='${process.env.XYZ_ROCK_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_METAL_CONTRACT_ADDRESS='${process.env.XYZ_METAL_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_ICE_CONTRACT_ADDRESS='${process.env.XYZ_ICE_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_GAS_CONTRACT_ADDRESS='${process.env.XYZ_GAS_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_WATER_CONTRACT_ADDRESS='${process.env.XYZ_WATER_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_GEM_CONTRACT_ADDRESS='${process.env.XYZ_GEM_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_LIFE_CONTRACT_ADDRESS='${process.env.XYZ_LIFE_CONTRACT_ADDRESS}'</script>
          <script>window.XYZ_XP_CONTRACT_ADDRESS='${process.env.XYZ_XP_CONTRACT_ADDRESS}'</script>
          <script>window.MARKETPLACE_CONTRACT_ADDRESS='${process.env.MARKETPLACE_CONTRACT_ADDRESS}'</script>
          <script>window.QUEST_CONTRACT_ADDRESS='${process.env.QUEST_CONTRACT_ADDRESS}'</script>
        </head>
        <body>
          <main id='content' />
          <script type='text/javascript' src='${buildManifestJSON[`client.js`]}'></script>
        </body>
      </html>
      `
    res.send(html)
  })

  app.listen(3000, () => {
    console.info(`Listening ${3000}`)
  })
}

listen()
