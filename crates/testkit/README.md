# nautilus-testkit

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-testkit)](https://docs.rs/nautilus-testkit/latest/nautilus-testkit/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-testkit.svg)](https://crates.io/crates/nautilus-testkit)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

Test utilities and data management for [NautilusTrader](http://nautilustrader.io).

The *testkit* crate provides comprehensive testing utilities including test data management,
file handling, and common testing patterns. This crate supports robust testing workflows
across the entire NautilusTrader ecosystem with automated data downloads and validation:

- **Test data management**: Automated downloading and caching of test datasets.
- **File utilities**: File integrity verification with SHA-256 checksums.
- **Path resolution**: Platform-agnostic test data path management.
- **Precision handling**: Support for both 64-bit and 128-bit precision test data.
- **Common patterns**: Reusable test utilities and helper functions.

## Platform

[NautilusTrader](http://nautilustrader.io) is an open-source, high-performance, production-grade
algorithmic trading platform, providing quantitative traders with the ability to backtest
portfolios of automated trading strategies on historical data with an event-driven engine,
and also deploy those same strategies live, with no code changes.

NautilusTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.

## Documentation

See [the docs](https://docs.rs/nautilus-testkit) for more detailed usage.

## License

The source code for NautilusTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md).

---

NautilusTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nautilustrader.io>.

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. All rights reserved.</span>
