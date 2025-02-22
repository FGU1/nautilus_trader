# Emulated Orders

The platform makes it possible to emulate most order types locally, regardless
of whether the type is supported on a trading venue. The logic and code paths for
order emulation are exactly the same for all [environment contexts](/concepts/architecture.md#environment-contexts)
and utilize a common `OrderEmulator` component.

:::note
There is no limitation on the number of emulated orders you can have per running instance.
:::

## Submitting for emulation

The only requirement to emulate an order is to pass a `TriggerType` to the `emulation_trigger`
parameter of an `Order` constructor, or `OrderFactory` creation method. The following
emulation trigger types are currently supported:
- `DEFAULT` (which is the same as `BID_ASK`)
- `BID_ASK` (emulated using quotes to trigger)
- `LAST` (emulated using trades to trigger)

Emulated orders are subject to the same risk controls as *regular* orders, and can be
modified and canceled by a trading strategy in the normal way. They will also be included
when canceling all orders.

:::info
An emulated order will retain its original client order ID throughout its entire life cycle, making it easy to query through the cache.
:::

## Life cycle

An emulated order will progress through the following stages:
- Submitted by a `Strategy` through the `submit_order` method.
- Then sent to the `RiskEngine` for pre-trade risk checks (it may be denied at this point).
- Then sent to the `OrderEmulator` where it is *held* / emulated.

### Held emulated orders

The following will occur for an emulated order now *held* by the `OrderEmulator` component:
- The original `SubmitOrder` command will be cached.
- The emulated order will be processed inside a local `MatchingCore` component.
- The `OrderEmulator` will subscribe to any needed market data (if not already) to update the matching core.
- The emulated order can be modified (by the trader) and updated (by the market) until *released* or canceled.

### Released emulated orders

Once an emulated order is triggered / matched locally based on the arrival of data, the following
*release* actions will occur:

- The order will be transformed to either a `MARKET` or `LIMIT` order (see below table) through an additional `OrderInitialized` event.
- The orders `emulation_trigger` will be set to `NONE` (it will no longer be treated as an emulated order by any component).
- The order attached to the original `SubmitOrder` command will be sent back to the `RiskEngine` for additional checks since any modification/updates.
- If not denied, then the command will continue to the `ExecutionEngine` and on to the trading venue via an `ExecutionClient` as normal.

The following table lists which order types are possible to emulate, and
which order type they transform to when being released for submission to the
trading venue.

## Order types

|                        | Can emulate | Released type |
|------------------------|-------------|---------------|
| `MARKET`               |             | n/a           |
| `MARKET_TO_LIMIT`      |             | n/a           |
| `LIMIT`                | ✓           | `MARKET`      |
| `STOP_MARKET`          | ✓           | `MARKET`      |
| `STOP_LIMIT`           | ✓           | `LIMIT`       |
| `MARKET_IF_TOUCHED`    | ✓           | `MARKET`      |
| `LIMIT_IF_TOUCHED`     | ✓           | `LIMIT`       |
| `TRAILING_STOP_MARKET` | ✓           | `MARKET`      |
| `TRAILING_STOP_LIMIT`  | ✓           | `LIMIT`       |

## Querying

When writing trading strategies, it may be necessary to know the state of emulated orders in the system.
It's possible to query for emulated orders through the following `Cache` methods:
- `self.cache.orders_emulated(...)`
- `self.cache.is_order_emulated(...)`
- `self.cache.orders_emulated_count(...)`

See the full [API reference](../../api_reference/cache) for additional details.

You can also query order objects directly:
- `order.is_emulated`

If either of these return `False`, then the order has been *released* from the
`OrderEmulator`, and so is no longer considered an emulated order (or was never an emulated order).

:::warning
It's not advised to hold a local reference to an emulated order, as the order
object will be transformed when/if the emulated order is *released*. You should rely
on the `Cache` which is made for the job.
:::

## Persisted emulated orders

If a running system either crashes or shuts down with active emulated orders, then
they will be reloaded inside the `OrderEmulator` from any configured cache database.
