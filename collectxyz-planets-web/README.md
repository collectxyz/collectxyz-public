# collectxyz-planets-web

Repository hosting code for the planets layer 2 web application, built on xyz. 

You will need a file in the root of this directory called `.env`, with the following contents. Substitute values as needed for testnet, localterra, etc.
```
CAPTCHA_PUBLIC_KEY=6LcCrmQcAAAAAGbopk3EXKztcWSsGSXRmN7Wfjqy
CAPTCHA_URL=https://captcha.collectxyz.com
XYZ_CONTRACT_ADDRESS=terra192vcn2julwy7r2cd5prnkmnm3nxsts5sm32ygq
PLANETS_CONTRACT_ADDRESS=terra1ygy36rw9hdms5dpr5danf8y5yadyltdyr37zdm
RANDOMNESS_CONTRACT_ADDRESS=terra12wp8d2zy2vg8z0qxy7d2ka92l2q5k55830fvmg
BONUS_TOKEN_CONTRACT_ADDRESS=terra1jm9jv95f0w3e0kk3kxxvtqg50456mf2c92xd7f
RESOURCE_GATHERING_CONTRACT_ADDRESS=terra15q74xg3c9prn6snh7u9phxfmmjg2kka2g7y42l
XYZ_ROCK_CONTRACT_ADDRESS=terra10vqhxq78sddclrxf24cazc4w0hzksw3fkmre7w
XYZ_METAL_CONTRACT_ADDRESS=terra12cwh0fqwe6ua5vgqs5hcmt0erlyv06cr4ven2s
XYZ_ICE_CONTRACT_ADDRESS=terra1dhpps609ugytug8c4p9trp8tsdcf98mz64auwc
XYZ_GAS_CONTRACT_ADDRESS=terra1pfv7wwjp4qmhaqux9qfuu6ehjrhyqlqwypstwk
XYZ_WATER_CONTRACT_ADDRESS=terra1nzeaw7f53y40rkknhftt4x0l87wzuzlf428qgv
XYZ_GEM_CONTRACT_ADDRESS=terra17qae29qaluy074zp00qj9l9jvmevs6hwjy3nlv
XYZ_LIFE_CONTRACT_ADDRESS=terra1fuw2g0wy8a8scdtlmm2tv38fucdqms9qgpsy6n
XYZ_XP_CONTRACT_ADDRESS=terra14mlj7cr493ym4mxfy3uk0dzm4wkdeslgn26x5g
MARKETPLACE_CONTRACT_ADDRESS=terra14fdyrkx6hhgh5pwx26ntaxvpy5funmyvg66hs8
QUEST_CONTRACT_ADDRESS=terra1ggway3uwuk6st0e0wemqwyma33ffysdncpqehp
```


For a production build:
1. `docker build .`
2. `docker run -p 3000:3000 [BUILD_DOCKER_IMAGE_GOES_HERE]`
3. visit localhost:3000 in your browser!

For a dev build:
1. `npm i`
2. `npm run build-server`
3. (continuously run) `npm run client:dev`
4. (continuously run in a separate terminal) `npm run server:dev`
5. (continuously run in a third terminal, optional) `npm run typecheck-watch`
6. visit localhost:3001 in your browser!
