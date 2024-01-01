# MMM! Backtest

This is a backtest server for Manifold Markets. It aims to match the original api endpoints, as defined [here](https://docs.manifold.markets/api), except with the backtest data.

## How to use

tbd ;)

## endpoint list

```
x  GET  /v0/user/[username]
x  GET  /v0/user/by-id/[id]
x  GET  /v0/me
x  GET  /v0/user/[username]/bets (Deprecated)
x  GET  /v0/groups
x  GET  /v0/group/[slug]
x  GET  /v0/group/by-id/[id]
x  GET  /v0/group/by-id/[id]/markets (Deprecated)
x  GET  /v0/markets
x  GET  /v0/market/[marketId]
x  GET  /v0/market/[marketId]/positions
x  GET  /v0/slug/[marketSlug]
x  GET  /v0/search-markets
x  GET  /v0/users
x  POST /v0/bet
x  POST /v0/bet/cancel/[id]
x  POST /v0/market
x  POST /v0/market/[marketId]/answer
x  POST /v0/market/[marketId]/add-liquidity
x  POST /v0/market/[marketId]/add-bounty
x  POST /v0/market/[marketId]/award-bounty
x  POST /v0/market/[marketId]/close
x  POST /v0/market/[marketId]/group
x  POST /v0/market/[marketId]/resolve
x  POST /v0/market/[marketId]/sell
x  POST /v0/sell-shares-dpm
x  POST /v0/comment
x  GET  /v0/comments
x  GET  /v0/bets
x  GET  /v0/managrams
x  POST /v0/managram
x  GET  /v0/leagues
```
