
1. Orderbook program — on-chain Anchor program, Alice places a sell order for AppleCoin at some price, Bob fills it. The program handles the atomic swap and emits an event (trade price, amounts, timestamp, maker/taker)

Full on chain matching
                                                                                                                                                                                                                                     
What does the program actually do?                                       

- place_order — escrows the tokens (locks funds in a PDA so they can't be spent elsewhere)
- fill_order — executes the atomic swap between maker and taker, emits TradeEvent
- cancel_order — returns escrowed tokens to the owner

Account structure
Each open order is a PDA storing: owner, base/quote mint, price, amount, side (buy/sell), status. When filled, the tokens move and the account can be closed.

2. Gothmog seeds — calls the program instructions to place + fill that one order, so you have a real trade transaction on the validator
3. Indexer — polls getSignaturesForAddress on the program, parses the emitted events from transaction logs, writes to SQLite
4. API — serves candle/trade history from SQLite   