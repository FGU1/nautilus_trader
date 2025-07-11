// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

// Under development
#![allow(dead_code)]
#![allow(unused_variables)]

//! Provides a `BacktestExecutionClient` implementation for backtesting.

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use nautilus_common::{
    cache::Cache,
    clock::Clock,
    messages::execution::{
        BatchCancelOrders, CancelAllOrders, CancelOrder, ModifyOrder, QueryOrder, SubmitOrder,
        SubmitOrderList, TradingCommand,
    },
};
use nautilus_core::UnixNanos;
use nautilus_execution::client::{ExecutionClient, base::BaseExecutionClient};
use nautilus_model::{
    accounts::AccountAny,
    enums::OmsType,
    identifiers::{AccountId, ClientId, TraderId, Venue},
    orders::Order,
    types::{AccountBalance, MarginBalance},
};

use crate::exchange::SimulatedExchange;

/// Execution client implementation for backtesting trading operations.
///
/// The `BacktestExecutionClient` provides an execution client interface for
/// backtesting environments, handling order management and trade execution
/// through simulated exchanges. It processes trading commands and coordinates
/// with the simulation infrastructure to provide realistic execution behavior.
pub struct BacktestExecutionClient {
    base: BaseExecutionClient,
    exchange: Rc<RefCell<SimulatedExchange>>,
    clock: Rc<RefCell<dyn Clock>>,
    is_connected: bool,
    routing: bool,
    frozen_account: bool,
}

impl Debug for BacktestExecutionClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(BacktestExecutionClient))
            .field("client_id", &self.base.client_id)
            .field("routing", &self.routing)
            .finish()
    }
}

impl BacktestExecutionClient {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trader_id: TraderId,
        account_id: AccountId,
        exchange: Rc<RefCell<SimulatedExchange>>,
        cache: Rc<RefCell<Cache>>,
        clock: Rc<RefCell<dyn Clock>>,
        routing: Option<bool>,
        frozen_account: Option<bool>,
    ) -> Self {
        let routing = routing.unwrap_or(false);
        let frozen_account = frozen_account.unwrap_or(false);
        let exchange_id = exchange.borrow().id;
        let base_client = BaseExecutionClient::new(
            trader_id,
            ClientId::from(exchange_id.as_str()),
            Venue::from(exchange_id.as_str()),
            exchange.borrow().oms_type,
            account_id,
            exchange.borrow().account_type,
            exchange.borrow().base_currency,
            clock.clone(),
            cache,
        );

        if !frozen_account {
            // TODO Register calculated account
        }

        Self {
            exchange,
            clock,
            base: base_client,
            is_connected: false,
            routing,
            frozen_account,
        }
    }
}

impl ExecutionClient for BacktestExecutionClient {
    fn is_connected(&self) -> bool {
        self.is_connected
    }

    fn client_id(&self) -> ClientId {
        self.base.client_id
    }

    fn account_id(&self) -> AccountId {
        self.base.account_id
    }

    fn venue(&self) -> Venue {
        self.base.venue
    }

    fn oms_type(&self) -> OmsType {
        self.base.oms_type
    }

    fn get_account(&self) -> Option<AccountAny> {
        self.base.get_account()
    }

    fn generate_account_state(
        &self,
        balances: Vec<AccountBalance>,
        margins: Vec<MarginBalance>,
        reported: bool,
        ts_event: UnixNanos,
    ) -> anyhow::Result<()> {
        self.base
            .generate_account_state(balances, margins, reported, ts_event)
    }

    fn start(&mut self) -> anyhow::Result<()> {
        self.is_connected = true;
        log::info!("Backtest execution client started");
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        self.is_connected = false;
        log::info!("Backtest execution client stopped");
        Ok(())
    }

    fn submit_order(&self, cmd: &SubmitOrder) -> anyhow::Result<()> {
        self.base.generate_order_submitted(
            cmd.strategy_id,
            cmd.instrument_id,
            cmd.client_order_id,
            self.clock.borrow().timestamp_ns(),
        );

        self.exchange
            .borrow_mut()
            .send(TradingCommand::SubmitOrder(cmd.clone())); // TODO: Remove this clone
        Ok(())
    }

    fn submit_order_list(&self, cmd: &SubmitOrderList) -> anyhow::Result<()> {
        for order in &cmd.order_list.orders {
            self.base.generate_order_submitted(
                cmd.strategy_id,
                order.instrument_id(),
                order.client_order_id(),
                self.clock.borrow().timestamp_ns(),
            );
        }

        self.exchange
            .borrow_mut()
            .send(TradingCommand::SubmitOrderList(cmd.clone()));
        Ok(())
    }

    fn modify_order(&self, cmd: &ModifyOrder) -> anyhow::Result<()> {
        self.exchange
            .borrow_mut()
            .send(TradingCommand::ModifyOrder(cmd.clone()));
        Ok(())
    }

    fn cancel_order(&self, cmd: &CancelOrder) -> anyhow::Result<()> {
        self.exchange
            .borrow_mut()
            .send(TradingCommand::CancelOrder(cmd.clone()));
        Ok(())
    }

    fn cancel_all_orders(&self, cmd: &CancelAllOrders) -> anyhow::Result<()> {
        self.exchange
            .borrow_mut()
            .send(TradingCommand::CancelAllOrders(cmd.clone()));
        Ok(())
    }

    fn batch_cancel_orders(&self, cmd: &BatchCancelOrders) -> anyhow::Result<()> {
        self.exchange
            .borrow_mut()
            .send(TradingCommand::BatchCancelOrders(cmd.clone()));
        Ok(())
    }

    fn query_order(&self, cmd: &QueryOrder) -> anyhow::Result<()> {
        self.exchange
            .borrow_mut()
            .send(TradingCommand::QueryOrder(cmd.clone()));
        Ok(())
    }
}
