# nautilus-live

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-live)](https://docs.rs/nautilus-live/latest/nautilus-live/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-live.svg)](https://crates.io/crates/nautilus-live)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)

Live system node for [NautilusTrader](http://nautilustrader.io).

The *live* crate provides high-level abstractions and infrastructure for running live trading
systems, including data streaming, execution management, and system lifecycle handling.
It builds on top of the system kernel to provide simplified interfaces for live deployment:

- `LiveNode` High-level abstraction for live system nodes.
- `LiveNodeConfig` Configuration for live node deployment.
- `AsyncRunner` for managing system real-time data flow.

## Platform

[NautilusTrader](http://nautilustrader.io) is an open-source, high-performance, production-grade
algorithmic trading platform, providing quantitative traders with the ability to backtest
portfolios of automated trading strategies on historical data with an event-driven engine,
and also deploy those same strategies live, with no code changes.

NautilusTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.

## Documentation

See [the docs](https://docs.rs/nautilus-live) for more detailed usage.

## License

The source code for NautilusTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md).

---

NautilusTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nautilustrader.io>.

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. All rights reserved.</span>
