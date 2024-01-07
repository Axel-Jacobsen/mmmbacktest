# MMM! Backtest

This is a backtest server for Manifold Markets. It aims to match the original api endpoints, as defined [here](https://docs.manifold.markets/api), except with the backtest data.

## TODOs

[warp](https://docs.rs/warp/latest/warp/) is really really good. The server stuff is dead easy. So all the work will be pulling data from the backtest data, filtering it to the [api specs](https://docs.manifold.markets/api), and returning it. I can add some stuff to simulate the markets over time to test bots that are time-sensitive. Also, adding a cli arg to download / update the backtest data would be good. But, you should also be able to call this from code, not just the cli, to make backtesting easy for bots.

I think the best method of loading / querying data will be to copy all the data into a sqlite db. If we are space constrained, we can delete the original json values.

`db.rs` will manage the sqlite connections.

## How to use

tbd ;)

## endpoint list

```
''  = not implemented yet
y   = implemented
n   = not going to implement
----------------------------

n  GET  /v0/user/[username]
n  GET  /v0/user/by-id/[id]
n  GET  /v0/me
n  GET  /v0/user/[username]/bets (Deprecated)
n  GET  /v0/groups
n  GET  /v0/group/[slug]
n  GET  /v0/group/by-id/[id]
n  GET  /v0/group/by-id/[id]/markets (Deprecated)
Y  GET  /v0/markets
Y  GET  /v0/market/[marketId]  // returns a LiteMarket instead of a FullMarket,
                               // since the backtest data includes LiteMarkets only
   GET  /v0/market/[marketId]/positions
   GET  /v0/slug/[marketSlug]
   GET  /v0/search-markets
n  GET  /v0/users
   POST /v0/bet
   POST /v0/bet/cancel/[id]
   POST /v0/market
   POST /v0/market/[marketId]/answer
   POST /v0/market/[marketId]/add-liquidity
   POST /v0/market/[marketId]/add-bounty
   POST /v0/market/[marketId]/award-bounty
   POST /v0/market/[marketId]/close
   POST /v0/market/[marketId]/group
   POST /v0/market/[marketId]/resolve
   POST /v0/market/[marketId]/sell
   POST /v0/sell-shares-dpm
n  POST /v0/comment
n  GET  /v0/comments
   GET  /v0/bets
n  GET  /v0/managrams
n  POST /v0/managram
n  GET  /v0/leagues
```
