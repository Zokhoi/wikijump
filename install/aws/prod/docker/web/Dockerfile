#
# Framerail build
#

FROM node:20-alpine

# Install pnpm
RUN npm install -g pnpm

# Copy sources
RUN mkdir /app
WORKDIR /app
COPY framerail/ ./
COPY assets ./src/assets

# Install app
RUN \
    pnpm install && \
    pnpm build && \
    pnpm prune --prod && \
    rm -rf src/ tests/

# Run command
USER node:node
ENV NODE_ENV=production
ENV FRAMERAIL_ENV=prod
EXPOSE 3000

CMD ["/usr/local/bin/node", "build"]
