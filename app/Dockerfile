FROM node:20.11-alpine AS build
WORKDIR /app/
COPY . .
RUN npm i
RUN npx next experimental-compile

FROM node:20.11-alpine
WORKDIR /app/

COPY --from=build /app/.next ./.next
COPY package.json .
COPY next.config.mjs .
RUN npm i --omit=dev

EXPOSE 3000
CMD npm run start