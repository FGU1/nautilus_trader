# nautilus-system

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-system)](https://docs.rs/nautilus-system/latest/nautilus-system/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-system.svg)](https://crates.io/crates/nautilus-system)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)

System-level components and orchestration for [NautilusTrader](http://nautilustrader.io).

The *system* crate provides the core system architecture for orchestrating trading systems,
including the kernel that manages all engines, configuration management,
and system-level factories for creating components:

- `NautilusKernel` - Core system orchestrator managing engines and components.
- `NautilusKernelConfig` - Configuration for kernel initialization.
- System builders and factories for component creation.

## Platform

[NautilusTrader](http://nautilustrader.io) is an open-source, high-performance, production-grade
algorithmic trading platform, providing quantitative traders with the ability to backtest
portfolios of automated trading strategies on historical data with an event-driven engine,
and also deploy those same strategies live, with no code changes.

NautilusTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.

## Documentation

See [the docs](https://docs.rs/nautilus-system) for more detailed usage.

## License

The source code for NautilusTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md).

---

NautilusTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nautilustrader.io>.

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. All rights reserved.</span>
