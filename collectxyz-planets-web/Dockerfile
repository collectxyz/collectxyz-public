# first build
FROM node:14.17.3 as builder

ENV APP_HOME /project

ARG BUILD_COMMAND=concurrent-build

WORKDIR $APP_HOME

COPY package*.json .env* ./
RUN npm install
RUN rm -f /root/.npmrc

COPY . .
RUN npm run $BUILD_COMMAND

RUN npm prune --production

# second stage: copy build and run
FROM node:14.17.3-alpine

ENV APP_HOME /project
ENV PORT 3000

WORKDIR $APP_HOME

COPY --from=builder /project/package*.json /project/.env* ./
COPY --from=builder /project/private/ ./private/
COPY --from=builder /project/public/ ./public/
COPY --from=builder /project/node_modules/ ./node_modules/

EXPOSE $PORT

CMD [ "npm", "run", "server:prod" ]
