FROM node:20-alpine AS builder

WORKDIR /app

ARG API_URL=http://rust-app:8080
ENV API_URL=$API_URL
ENV NEXT_PUBLIC_API_URL=/api
ENV NEXT_TELEMETRY_DISABLED=1

COPY package*.json ./

RUN npm ci

COPY . .

RUN npm run build && npm prune --omit=dev

FROM node:20-alpine AS runner

WORKDIR /app

ARG API_URL=http://rust-app:8080
ENV NODE_ENV=production
ENV API_URL=$API_URL
ENV NEXT_PUBLIC_API_URL=/api
ENV NEXT_TELEMETRY_DISABLED=1

COPY --from=builder /app/package*.json ./
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/public ./public

EXPOSE 3000

CMD ["npm", "run", "start"]
