# ── Build Stage ──
FROM node:20-slim AS builder

RUN corepack enable && corepack prepare pnpm@9 --activate

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

# ── Runtime Stage ──
FROM node:20-slim

RUN corepack enable && corepack prepare pnpm@9 --activate

WORKDIR /app

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=builder /app/node_modules ./node_modules

EXPOSE 3000

ENV NODE_ENV=production
ENV PORT=3000
ENV ORIGIN=https://app.fieldforge.com

CMD ["node", "build"]
