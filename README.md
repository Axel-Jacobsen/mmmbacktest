# MMM! Backtest

This is a backtest server for Manifold Markets. It aims to match the original api endpoints, as defined [here](https://docs.manifold.markets/api), except with the backtest data.

## TODOs

[warp](https://docs.rs/warp/latest/warp/) is really really good. The server stuff is dead easy. So all the work will be pulling data from the backtest data, filtering it to the [api specs](https://docs.manifold.markets/api), and returning it. I can add some stuff to simulate the markets over time to test bots that are time-sensitive. Also, adding a cli arg to download / update the backtest data would be good. But, you should also be able to call this from code, not just the cli, to make backtesting easy for bots.

- cli (refine)
- rust frontend
- python bindings
- auto download / setup

## How to use

tbd ;)

## endpoint list

```
''  = not implemented yet
y   = implemented
n   = not going to implement
----------------------------

   GET  /v0/user/[username]                         // we have no user data
   GET  /v0/user/by-id/[id]
   GET  /v0/me
n  GET  /v0/user/[username]/bets (Deprecated)
n  GET  /v0/groups                                  // we have no group data
n  GET  /v0/group/[slug]
n  GET  /v0/group/by-id/[id]
n  GET  /v0/group/by-id/[id]/markets (Deprecated)
Y  GET  /v0/markets
Y  GET  /v0/market/[marketId]                       // returns a LiteMarket instead of a FullMarket, since the backtest data includes LiteMarkets only
n  GET  /v0/market/[marketId]/positions             // returns type ContractMetrics, which we don't have
Y  GET  /v0/slug/[marketSlug]
   GET  /v0/search-markets
n  GET  /v0/users
   POST /v0/bet
   POST /v0/bet/cancel/[id]
n  POST /v0/market
n  POST /v0/market/[marketId]/answer
n  POST /v0/market/[marketId]/add-liquidity
n  POST /v0/market/[marketId]/add-bounty
n  POST /v0/market/[marketId]/award-bounty
n  POST /v0/market/[marketId]/close
n  POST /v0/market/[marketId]/group
n  POST /v0/market/[marketId]/resolve
   POST /v0/market/[marketId]/sell
   POST /v0/sell-shares-dpm
n  POST /v0/comment
n  GET  /v0/comments
Y  GET  /v0/bets
n  GET  /v0/managrams
n  POST /v0/managram
n  GET  /v0/leagues
```
