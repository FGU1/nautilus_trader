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

use std::{any::Any, num::NonZeroUsize, sync::Arc};

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use nautilus_core::{UUID4, UnixNanos};
use nautilus_model::{
    data::{Bar, BarType, DataType, QuoteTick, TradeTick},
    enums::BookType,
    identifiers::{ClientId, InstrumentId, Venue},
    instruments::InstrumentAny,
    orderbook::OrderBook,
};

#[derive(Clone, Debug, PartialEq)]
pub enum DataCommand {
    Request(RequestCommand),
    Subscribe(SubscribeCommand),
    Unsubscribe(UnsubscribeCommand),
}

impl DataCommand {
    /// Converts the command to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub enum SubscribeCommand {
    Data(SubscribeData),
    Instruments(SubscribeInstruments),
    Instrument(SubscribeInstrument),
    BookDeltas(SubscribeBookDeltas),
    BookDepth10(SubscribeBookDepth10),
    BookSnapshots(SubscribeBookSnapshots),
    Quotes(SubscribeQuotes),
    Trades(SubscribeTrades),
    Bars(SubscribeBars),
    MarkPrices(SubscribeMarkPrices),
    IndexPrices(SubscribeIndexPrices),
    InstrumentStatus(SubscribeInstrumentStatus),
    InstrumentClose(SubscribeInstrumentClose),
}

impl PartialEq for SubscribeCommand {
    fn eq(&self, other: &Self) -> bool {
        self.command_id() == other.command_id()
    }
}

impl SubscribeCommand {
    /// Converts the command to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn command_id(&self) -> UUID4 {
        match self {
            Self::Data(cmd) => cmd.command_id,
            Self::Instruments(cmd) => cmd.command_id,
            Self::Instrument(cmd) => cmd.command_id,
            Self::BookDeltas(cmd) => cmd.command_id,
            Self::BookDepth10(cmd) => cmd.command_id,
            Self::BookSnapshots(cmd) => cmd.command_id,
            Self::Quotes(cmd) => cmd.command_id,
            Self::Trades(cmd) => cmd.command_id,
            Self::Bars(cmd) => cmd.command_id,
            Self::MarkPrices(cmd) => cmd.command_id,
            Self::IndexPrices(cmd) => cmd.command_id,
            Self::InstrumentStatus(cmd) => cmd.command_id,
            Self::InstrumentClose(cmd) => cmd.command_id,
        }
    }

    pub fn client_id(&self) -> Option<&ClientId> {
        match self {
            Self::Data(cmd) => cmd.client_id.as_ref(),
            Self::Instruments(cmd) => cmd.client_id.as_ref(),
            Self::Instrument(cmd) => cmd.client_id.as_ref(),
            Self::BookDeltas(cmd) => cmd.client_id.as_ref(),
            Self::BookDepth10(cmd) => cmd.client_id.as_ref(),
            Self::BookSnapshots(cmd) => cmd.client_id.as_ref(),
            Self::Quotes(cmd) => cmd.client_id.as_ref(),
            Self::Trades(cmd) => cmd.client_id.as_ref(),
            Self::MarkPrices(cmd) => cmd.client_id.as_ref(),
            Self::IndexPrices(cmd) => cmd.client_id.as_ref(),
            Self::Bars(cmd) => cmd.client_id.as_ref(),
            Self::InstrumentStatus(cmd) => cmd.client_id.as_ref(),
            Self::InstrumentClose(cmd) => cmd.client_id.as_ref(),
        }
    }

    pub fn venue(&self) -> Option<&Venue> {
        match self {
            Self::Data(cmd) => cmd.venue.as_ref(),
            Self::Instruments(cmd) => Some(&cmd.venue),
            Self::Instrument(cmd) => cmd.venue.as_ref(),
            Self::BookDeltas(cmd) => cmd.venue.as_ref(),
            Self::BookDepth10(cmd) => cmd.venue.as_ref(),
            Self::BookSnapshots(cmd) => cmd.venue.as_ref(),
            Self::Quotes(cmd) => cmd.venue.as_ref(),
            Self::Trades(cmd) => cmd.venue.as_ref(),
            Self::MarkPrices(cmd) => cmd.venue.as_ref(),
            Self::IndexPrices(cmd) => cmd.venue.as_ref(),
            Self::Bars(cmd) => cmd.venue.as_ref(),
            Self::InstrumentStatus(cmd) => cmd.venue.as_ref(),
            Self::InstrumentClose(cmd) => cmd.venue.as_ref(),
        }
    }

    pub fn ts_init(&self) -> UnixNanos {
        match self {
            Self::Data(cmd) => cmd.ts_init,
            Self::Instruments(cmd) => cmd.ts_init,
            Self::Instrument(cmd) => cmd.ts_init,
            Self::BookDeltas(cmd) => cmd.ts_init,
            Self::BookDepth10(cmd) => cmd.ts_init,
            Self::BookSnapshots(cmd) => cmd.ts_init,
            Self::Quotes(cmd) => cmd.ts_init,
            Self::Trades(cmd) => cmd.ts_init,
            Self::MarkPrices(cmd) => cmd.ts_init,
            Self::IndexPrices(cmd) => cmd.ts_init,
            Self::Bars(cmd) => cmd.ts_init,
            Self::InstrumentStatus(cmd) => cmd.ts_init,
            Self::InstrumentClose(cmd) => cmd.ts_init,
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnsubscribeCommand {
    Data(UnsubscribeData),
    Instruments(UnsubscribeInstruments),
    Instrument(UnsubscribeInstrument),
    BookDeltas(UnsubscribeBookDeltas),
    BookDepth10(UnsubscribeBookDepth10),
    BookSnapshots(UnsubscribeBookSnapshots),
    Quotes(UnsubscribeQuotes),
    Trades(UnsubscribeTrades),
    Bars(UnsubscribeBars),
    MarkPrices(UnsubscribeMarkPrices),
    IndexPrices(UnsubscribeIndexPrices),
    InstrumentStatus(UnsubscribeInstrumentStatus),
    InstrumentClose(UnsubscribeInstrumentClose),
}

impl PartialEq for UnsubscribeCommand {
    fn eq(&self, other: &Self) -> bool {
        self.command_id() == other.command_id()
    }
}

impl UnsubscribeCommand {
    /// Converts the command to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn command_id(&self) -> UUID4 {
        match self {
            Self::Data(cmd) => cmd.command_id,
            Self::Instruments(cmd) => cmd.command_id,
            Self::Instrument(cmd) => cmd.command_id,
            Self::BookDeltas(cmd) => cmd.command_id,
            Self::BookDepth10(cmd) => cmd.command_id,
            Self::BookSnapshots(cmd) => cmd.command_id,
            Self::Quotes(cmd) => cmd.command_id,
            Self::Trades(cmd) => cmd.command_id,
            Self::Bars(cmd) => cmd.command_id,
            Self::MarkPrices(cmd) => cmd.command_id,
            Self::IndexPrices(cmd) => cmd.command_id,
            Self::InstrumentStatus(cmd) => cmd.command_id,
            Self::InstrumentClose(cmd) => cmd.command_id,
        }
    }

    pub fn client_id(&self) -> Option<&ClientId> {
        match self {
            Self::Data(cmd) => cmd.client_id.as_ref(),
            Self::Instruments(cmd) => cmd.client_id.as_ref(),
            Self::Instrument(cmd) => cmd.client_id.as_ref(),
            Self::BookDeltas(cmd) => cmd.client_id.as_ref(),
            Self::BookDepth10(cmd) => cmd.client_id.as_ref(),
            Self::BookSnapshots(cmd) => cmd.client_id.as_ref(),
            Self::Quotes(cmd) => cmd.client_id.as_ref(),
            Self::Trades(cmd) => cmd.client_id.as_ref(),
            Self::Bars(cmd) => cmd.client_id.as_ref(),
            Self::MarkPrices(cmd) => cmd.client_id.as_ref(),
            Self::IndexPrices(cmd) => cmd.client_id.as_ref(),
            Self::InstrumentStatus(cmd) => cmd.client_id.as_ref(),
            Self::InstrumentClose(cmd) => cmd.client_id.as_ref(),
        }
    }

    pub fn venue(&self) -> Option<&Venue> {
        match self {
            Self::Data(cmd) => cmd.venue.as_ref(),
            Self::Instruments(cmd) => Some(&cmd.venue),
            Self::Instrument(cmd) => cmd.venue.as_ref(),
            Self::BookDeltas(cmd) => cmd.venue.as_ref(),
            Self::BookDepth10(cmd) => cmd.venue.as_ref(),
            Self::BookSnapshots(cmd) => cmd.venue.as_ref(),
            Self::Quotes(cmd) => cmd.venue.as_ref(),
            Self::Trades(cmd) => cmd.venue.as_ref(),
            Self::Bars(cmd) => cmd.venue.as_ref(),
            Self::MarkPrices(cmd) => cmd.venue.as_ref(),
            Self::IndexPrices(cmd) => cmd.venue.as_ref(),
            Self::InstrumentStatus(cmd) => cmd.venue.as_ref(),
            Self::InstrumentClose(cmd) => cmd.venue.as_ref(),
        }
    }

    pub fn ts_init(&self) -> UnixNanos {
        match self {
            Self::Data(cmd) => cmd.ts_init,
            Self::Instruments(cmd) => cmd.ts_init,
            Self::Instrument(cmd) => cmd.ts_init,
            Self::BookDeltas(cmd) => cmd.ts_init,
            Self::BookDepth10(cmd) => cmd.ts_init,
            Self::BookSnapshots(cmd) => cmd.ts_init,
            Self::Quotes(cmd) => cmd.ts_init,
            Self::Trades(cmd) => cmd.ts_init,
            Self::MarkPrices(cmd) => cmd.ts_init,
            Self::IndexPrices(cmd) => cmd.ts_init,
            Self::Bars(cmd) => cmd.ts_init,
            Self::InstrumentStatus(cmd) => cmd.ts_init,
            Self::InstrumentClose(cmd) => cmd.ts_init,
        }
    }
}

fn check_client_id_or_venue(client_id: &Option<ClientId>, venue: &Option<Venue>) {
    assert!(
        client_id.is_some() || venue.is_some(),
        "Both `client_id` and `venue` were None"
    );
}

#[derive(Clone, Debug)]
pub struct SubscribeData {
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub data_type: DataType,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeData {
    pub fn new(
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        data_type: DataType,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            client_id,
            venue,
            data_type,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeInstruments {
    pub client_id: Option<ClientId>,
    pub venue: Venue,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeInstruments {
    pub fn new(
        client_id: Option<ClientId>,
        venue: Venue,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeInstrument {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeInstrument {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeBookDeltas {
    pub instrument_id: InstrumentId,
    pub book_type: BookType,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub depth: Option<NonZeroUsize>,
    pub managed: bool,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeBookDeltas {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        book_type: BookType,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        depth: Option<NonZeroUsize>,
        managed: bool,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            book_type,
            client_id,
            venue,
            command_id,
            ts_init,
            depth,
            managed,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeBookDepth10 {
    pub instrument_id: InstrumentId,
    pub book_type: BookType,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub depth: Option<NonZeroUsize>,
    pub managed: bool,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeBookDepth10 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        book_type: BookType,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        depth: Option<NonZeroUsize>,
        managed: bool,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            book_type,
            client_id,
            venue,
            command_id,
            ts_init,
            depth,
            managed,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeBookSnapshots {
    pub instrument_id: InstrumentId,
    pub book_type: BookType,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub depth: Option<NonZeroUsize>,
    pub interval_ms: NonZeroUsize,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeBookSnapshots {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        book_type: BookType,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        depth: Option<NonZeroUsize>,
        interval_ms: NonZeroUsize,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            book_type,
            client_id,
            venue,
            command_id,
            ts_init,
            depth,
            interval_ms,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeQuotes {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeQuotes {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeTrades {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeTrades {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeBars {
    pub bar_type: BarType,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub await_partial: bool,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeBars {
    pub fn new(
        bar_type: BarType,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        await_partial: bool,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            bar_type,
            client_id,
            venue,
            command_id,
            ts_init,
            await_partial,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeMarkPrices {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeMarkPrices {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeIndexPrices {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeIndexPrices {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeInstrumentStatus {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeInstrumentStatus {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeInstrumentClose {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl SubscribeInstrumentClose {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeData {
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub data_type: DataType,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeData {
    pub fn new(
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        data_type: DataType,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            client_id,
            venue,
            data_type,
            command_id,
            ts_init,
            params,
        }
    }
}

// Unsubscribe commands
#[derive(Clone, Debug)]
pub struct UnsubscribeInstruments {
    pub client_id: Option<ClientId>,
    pub venue: Venue,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeInstruments {
    pub fn new(
        client_id: Option<ClientId>,
        venue: Venue,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeInstrument {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeInstrument {
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeBookDeltas {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeBookDeltas {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeBookDepth10 {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeBookDepth10 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeBookSnapshots {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeBookSnapshots {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeQuotes {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeQuotes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeTrades {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeTrades {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeBars {
    pub bar_type: BarType,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeBars {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bar_type: BarType,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            bar_type,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeMarkPrices {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeMarkPrices {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeIndexPrices {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeIndexPrices {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeInstrumentStatus {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeInstrumentStatus {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnsubscribeInstrumentClose {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl UnsubscribeInstrumentClose {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        command_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            instrument_id,
            client_id,
            venue,
            command_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub enum RequestCommand {
    Data(RequestData),
    Instrument(RequestInstrument),
    Instruments(RequestInstruments),
    BookSnapshot(RequestBookSnapshot),
    Quotes(RequestQuotes),
    Trades(RequestTrades),
    Bars(RequestBars),
}

impl PartialEq for RequestCommand {
    fn eq(&self, other: &Self) -> bool {
        self.request_id() == other.request_id()
    }
}

impl RequestCommand {
    /// Converts the command to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn request_id(&self) -> &UUID4 {
        match self {
            Self::Data(cmd) => &cmd.request_id,
            Self::Instruments(cmd) => &cmd.request_id,
            Self::Instrument(cmd) => &cmd.request_id,
            Self::BookSnapshot(cmd) => &cmd.request_id,
            Self::Quotes(cmd) => &cmd.request_id,
            Self::Trades(cmd) => &cmd.request_id,
            Self::Bars(cmd) => &cmd.request_id,
        }
    }

    pub fn client_id(&self) -> Option<&ClientId> {
        match self {
            Self::Data(cmd) => Some(&cmd.client_id),
            Self::Instruments(cmd) => cmd.client_id.as_ref(),
            Self::Instrument(cmd) => cmd.client_id.as_ref(),
            Self::BookSnapshot(cmd) => cmd.client_id.as_ref(),
            Self::Quotes(cmd) => cmd.client_id.as_ref(),
            Self::Trades(cmd) => cmd.client_id.as_ref(),
            Self::Bars(cmd) => cmd.client_id.as_ref(),
        }
    }

    pub fn venue(&self) -> Option<&Venue> {
        match self {
            Self::Data(_) => None,
            Self::Instruments(cmd) => cmd.venue.as_ref(),
            Self::Instrument(cmd) => Some(&cmd.instrument_id.venue),
            Self::BookSnapshot(cmd) => Some(&cmd.instrument_id.venue),
            Self::Quotes(cmd) => Some(&cmd.instrument_id.venue),
            Self::Trades(cmd) => Some(&cmd.instrument_id.venue),
            // TODO: Extract the below somewhere
            Self::Bars(cmd) => match &cmd.bar_type {
                BarType::Standard { instrument_id, .. } => Some(&instrument_id.venue),
                BarType::Composite { instrument_id, .. } => Some(&instrument_id.venue),
            },
        }
    }

    pub fn ts_init(&self) -> UnixNanos {
        match self {
            Self::Data(cmd) => cmd.ts_init,
            Self::Instruments(cmd) => cmd.ts_init,
            Self::Instrument(cmd) => cmd.ts_init,
            Self::BookSnapshot(cmd) => cmd.ts_init,
            Self::Quotes(cmd) => cmd.ts_init,
            Self::Trades(cmd) => cmd.ts_init,
            Self::Bars(cmd) => cmd.ts_init,
        }
    }
}

// Request data structures
#[derive(Clone, Debug)]
pub struct RequestInstrument {
    pub instrument_id: InstrumentId,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub client_id: Option<ClientId>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

#[derive(Clone, Debug)]
pub struct RequestData {
    pub client_id: ClientId,
    pub data_type: DataType,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestInstrument {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        client_id: Option<ClientId>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            instrument_id,
            start,
            end,
            client_id,
            request_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestInstruments {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub client_id: Option<ClientId>,
    pub venue: Option<Venue>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestInstruments {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        client_id: Option<ClientId>,
        venue: Option<Venue>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        check_client_id_or_venue(&client_id, &venue);
        Self {
            start,
            end,
            client_id,
            venue,
            request_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestBookSnapshot {
    pub instrument_id: InstrumentId,
    pub depth: Option<NonZeroUsize>,
    pub client_id: Option<ClientId>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestBookSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        depth: Option<NonZeroUsize>,
        client_id: Option<ClientId>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            instrument_id,
            depth,
            client_id,
            request_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestQuotes {
    pub instrument_id: InstrumentId,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<NonZeroUsize>,
    pub client_id: Option<ClientId>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestQuotes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        limit: Option<NonZeroUsize>,
        client_id: Option<ClientId>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            instrument_id,
            start,
            end,
            limit,
            client_id,
            request_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestTrades {
    pub instrument_id: InstrumentId,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<NonZeroUsize>,
    pub client_id: Option<ClientId>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestTrades {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: InstrumentId,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        limit: Option<NonZeroUsize>,
        client_id: Option<ClientId>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            instrument_id,
            start,
            end,
            limit,
            client_id,
            request_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestBars {
    pub bar_type: BarType,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<NonZeroUsize>,
    pub client_id: Option<ClientId>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestBars {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bar_type: BarType,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        limit: Option<NonZeroUsize>,
        client_id: Option<ClientId>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            bar_type,
            start,
            end,
            limit,
            client_id,
            request_id,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DataResponse {
    Data(CustomDataResponse),
    Instrument(Box<InstrumentResponse>),
    Instruments(InstrumentsResponse),
    Book(BookResponse),
    Quotes(QuotesResponse),
    Trades(TradesResponse),
    Bars(BarsResponse),
}

impl DataResponse {
    /// Converts the command to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn correlation_id(&self) -> &UUID4 {
        match self {
            Self::Data(resp) => &resp.correlation_id,
            Self::Instrument(resp) => &resp.correlation_id,
            Self::Instruments(resp) => &resp.correlation_id,
            Self::Book(resp) => &resp.correlation_id,
            Self::Quotes(resp) => &resp.correlation_id,
            Self::Trades(resp) => &resp.correlation_id,
            Self::Bars(resp) => &resp.correlation_id,
        }
    }
}

pub type Payload = Arc<dyn Any + Send + Sync>;

#[derive(Clone, Debug)]
pub struct CustomDataResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub venue: Option<Venue>,
    pub data_type: DataType,
    pub data: Payload,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl CustomDataResponse {
    pub fn new<T: Any + Send + Sync>(
        correlation_id: UUID4,
        client_id: ClientId,
        venue: Option<Venue>,
        data_type: DataType,
        data: T,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            venue,
            data_type,
            data: Arc::new(data),
            ts_init,
            params,
        }
    }

    /// Converts the response to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct InstrumentResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub instrument_id: InstrumentId,
    pub data: InstrumentAny,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl InstrumentResponse {
    /// Converts to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn new(
        correlation_id: UUID4,
        client_id: ClientId,
        instrument_id: InstrumentId,
        data: InstrumentAny,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            instrument_id,
            data,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstrumentsResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub venue: Venue,
    pub data: Vec<InstrumentAny>,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl InstrumentsResponse {
    /// Converts to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn new(
        correlation_id: UUID4,
        client_id: ClientId,
        venue: Venue,
        data: Vec<InstrumentAny>,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            venue,
            data,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BookResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub instrument_id: InstrumentId,
    pub data: OrderBook,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl BookResponse {
    /// Converts to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn new(
        correlation_id: UUID4,
        client_id: ClientId,
        instrument_id: InstrumentId,
        data: OrderBook,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            instrument_id,
            data,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct QuotesResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub instrument_id: InstrumentId,
    pub data: Vec<QuoteTick>,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl QuotesResponse {
    /// Converts to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn new(
        correlation_id: UUID4,
        client_id: ClientId,
        instrument_id: InstrumentId,
        data: Vec<QuoteTick>,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            instrument_id,
            data,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TradesResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub instrument_id: InstrumentId,
    pub data: Vec<TradeTick>,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl TradesResponse {
    /// Converts to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn new(
        correlation_id: UUID4,
        client_id: ClientId,
        instrument_id: InstrumentId,
        data: Vec<TradeTick>,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            instrument_id,
            data,
            ts_init,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BarsResponse {
    pub correlation_id: UUID4,
    pub client_id: ClientId,
    pub bar_type: BarType,
    pub data: Vec<Bar>,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl BarsResponse {
    /// Converts to a dyn Any trait object for messaging.
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn new(
        correlation_id: UUID4,
        client_id: ClientId,
        bar_type: BarType,
        data: Vec<Bar>,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            correlation_id,
            client_id,
            bar_type,
            data,
            ts_init,
            params,
        }
    }
}
