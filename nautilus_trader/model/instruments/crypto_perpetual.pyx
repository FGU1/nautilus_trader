# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from decimal import Decimal

from libc.stdint cimport uint64_t

from nautilus_trader.core.correctness cimport Condition
from nautilus_trader.core.rust.model cimport AssetClass
from nautilus_trader.core.rust.model cimport InstrumentClass
from nautilus_trader.model.identifiers cimport InstrumentId
from nautilus_trader.model.identifiers cimport Symbol
from nautilus_trader.model.instruments.base cimport Instrument
from nautilus_trader.model.objects cimport Currency
from nautilus_trader.model.objects cimport Money
from nautilus_trader.model.objects cimport Price
from nautilus_trader.model.objects cimport Quantity


cdef class CryptoPerpetual(Instrument):
    """
    Represents a crypto perpetual futures contract instrument (a.k.a. perpetual swap).

    Parameters
    ----------
    instrument_id : InstrumentId
        The instrument ID for the instrument.
    raw_symbol : Symbol
        The raw/local/native symbol for the instrument, assigned by the venue.
    base_currency : Currency, optional
        The base currency.
    quote_currency : Currency
        The quote currency.
    settlement_currency : Currency
        The settlement currency.
    is_inverse : bool
        If the instrument costing is inverse (quantity expressed in quote currency units).
    price_precision : int
        The price decimal precision.
    size_precision : int
        The trading size decimal precision.
    price_increment : Price
        The minimum price increment (tick size).
    size_increment : Quantity
        The minimum size increment.
    ts_event : uint64_t
        UNIX timestamp (nanoseconds) when the data event occurred.
    ts_init : uint64_t
        UNIX timestamp (nanoseconds) when the data object was initialized.
    multiplier : Quantity, default 1
        The contract multiplier.
    max_quantity : Quantity, optional
        The maximum allowable order quantity.
    min_quantity : Quantity, optional
        The minimum allowable order quantity.
    max_notional : Money, optional
        The maximum allowable order notional value.
    min_notional : Money, optional
        The minimum allowable order notional value.
    max_price : Price, optional
        The maximum allowable quoted price.
    min_price : Price, optional
        The minimum allowable quoted price.
    margin_init : Decimal, optional
        The initial (order) margin requirement in percentage of order value.
    margin_maint : Decimal, optional
        The maintenance (position) margin in percentage of position value.
    maker_fee : Decimal, optional
        The fee rate for liquidity makers as a percentage of order value.
    taker_fee : Decimal, optional
        The fee rate for liquidity takers as a percentage of order value.
    info : dict[str, object], optional
        The additional instrument information.

    Raises
    ------
    ValueError
        If `price_precision` is negative (< 0).
    ValueError
        If `size_precision` is negative (< 0).
    ValueError
        If `price_increment` is not positive (> 0).
    ValueError
        If `size_increment` is not positive (> 0).
    ValueError
        If `price_precision` is not equal to price_increment.precision.
    ValueError
        If `size_increment` is not equal to size_increment.precision.
    ValueError
        If `multiplier` is not positive (> 0).
    ValueError
        If `max_quantity` is not positive (> 0).
    ValueError
        If `min_quantity` is negative (< 0).
    ValueError
        If `max_notional` is not positive (> 0).
    ValueError
        If `min_notional` is negative (< 0).
    ValueError
        If `max_price` is not positive (> 0).
    ValueError
        If `min_price` is negative (< 0).
    ValueError
        If `margin_init` is negative (< 0).
    ValueError
        If `margin_maint` is negative (< 0).

    """

    def __init__(
        self,
        InstrumentId instrument_id not None,
        Symbol raw_symbol not None,
        Currency base_currency not None,
        Currency quote_currency not None,
        Currency settlement_currency not None,
        bint is_inverse,
        int price_precision,
        int size_precision,
        Price price_increment not None,
        Quantity size_increment not None,
        uint64_t ts_event,
        uint64_t ts_init,
        multiplier=Quantity.from_int_c(1),
        Quantity max_quantity: Quantity | None = None,
        Quantity min_quantity: Quantity | None = None,
        Money max_notional: Money | None = None,
        Money min_notional: Money | None = None,
        Price max_price: Price | None = None,
        Price min_price: Price | None = None,
        margin_init: Decimal | None = None,
        margin_maint: Decimal | None = None,
        maker_fee: Decimal | None = None,
        taker_fee: Decimal | None = None,
        dict info = None,
    ) -> None:
        super().__init__(
            instrument_id=instrument_id,
            raw_symbol=raw_symbol,
            asset_class=AssetClass.CRYPTOCURRENCY,
            instrument_class=InstrumentClass.SWAP,
            quote_currency=quote_currency,
            is_inverse=is_inverse,
            price_precision=price_precision,
            size_precision=size_precision,
            price_increment=price_increment,
            size_increment=size_increment,
            multiplier=multiplier,
            lot_size=Quantity.from_int_c(1),
            max_quantity=max_quantity,
            min_quantity=min_quantity,
            max_notional=max_notional,
            min_notional=min_notional,
            max_price=max_price,
            min_price=min_price,
            margin_init=margin_init or Decimal(0),
            margin_maint=margin_maint or Decimal(0),
            maker_fee=maker_fee or Decimal(0),
            taker_fee=taker_fee or Decimal(0),
            ts_event=ts_event,
            ts_init=ts_init,
            info=info,
        )

        self.base_currency = base_currency
        self.settlement_currency = settlement_currency
        if settlement_currency != base_currency and settlement_currency != quote_currency:
            self.is_quanto = True
        else:
            self.is_quanto = False

    cpdef Currency get_base_currency(self):
        """
        Return the instruments base currency.

        Returns
        -------
        Currency

        """
        return self.base_currency

    cpdef Currency get_settlement_currency(self):
        """
        Return the currency used to settle a trade of the instrument.

        Returns
        -------
        Currency

        """
        return self.settlement_currency

    @staticmethod
    cdef CryptoPerpetual from_dict_c(dict values):
        Condition.not_none(values, "values")
        cdef str max_q = values["max_quantity"]
        cdef str min_q = values["min_quantity"]
        cdef str max_n = values["max_notional"]
        cdef str min_n = values["min_notional"]
        cdef str max_p = values["max_price"]
        cdef str min_p = values["min_price"]
        return CryptoPerpetual(
            instrument_id=InstrumentId.from_str_c(values["id"]),
            raw_symbol=Symbol(values["raw_symbol"]),
            base_currency=Currency.from_str_c(values["base_currency"]),
            quote_currency=Currency.from_str_c(values["quote_currency"]),
            settlement_currency=Currency.from_str_c(values["settlement_currency"]),
            is_inverse=values["is_inverse"],
            price_precision=values["price_precision"],
            size_precision=values["size_precision"],
            price_increment=Price.from_str_c(values["price_increment"]),
            size_increment=Quantity.from_str_c(values["size_increment"]),
            multiplier=Quantity.from_str_c(values["multiplier"]),
            max_quantity=Quantity.from_str_c(max_q) if max_q is not None else None,
            min_quantity=Quantity.from_str_c(min_q) if min_q is not None else None,
            max_notional=Money.from_str_c(max_n) if max_n is not None else None,
            min_notional=Money.from_str_c(min_n) if min_n is not None else None,
            max_price=Price.from_str_c(max_p) if max_p is not None else None,
            min_price=Price.from_str_c(min_p) if min_p is not None else None,
            margin_init=Decimal(values["margin_init"]),
            margin_maint=Decimal(values["margin_maint"]),
            maker_fee=Decimal(values["maker_fee"]),
            taker_fee=Decimal(values["taker_fee"]),
            ts_event=values["ts_event"],
            ts_init=values["ts_init"],
            info=values["info"],
        )

    @staticmethod
    cdef dict to_dict_c(CryptoPerpetual obj):
        Condition.not_none(obj, "obj")
        return {
            "type": "CryptoPerpetual",
            "id": obj.id.to_str(),
            "raw_symbol": obj.raw_symbol.to_str(),
            "base_currency": obj.base_currency.code,
            "quote_currency": obj.quote_currency.code,
            "settlement_currency": obj.settlement_currency.code,
            "is_inverse": obj.is_inverse,
            "price_precision": obj.price_precision,
            "price_increment": str(obj.price_increment),
            "size_precision": obj.size_precision,
            "size_increment": str(obj.size_increment),
            "multiplier": str(obj.multiplier),
            "lot_size": str(obj.lot_size),
            "max_quantity": str(obj.max_quantity) if obj.max_quantity is not None else None,
            "min_quantity": str(obj.min_quantity) if obj.min_quantity is not None else None,
            "max_notional": str(obj.max_notional) if obj.max_notional is not None else None,
            "min_notional": str(obj.min_notional) if obj.min_notional is not None else None,
            "max_price": str(obj.max_price) if obj.max_price is not None else None,
            "min_price": str(obj.min_price) if obj.min_price is not None else None,
            "margin_init": str(obj.margin_init),
            "margin_maint": str(obj.margin_maint),
            "maker_fee": str(obj.maker_fee),
            "taker_fee": str(obj.taker_fee),
            "ts_event": obj.ts_event,
            "ts_init": obj.ts_init,
            "info": obj.info,
        }

    @staticmethod
    def from_dict(dict values) -> CryptoPerpetual:
        """
        Return an instrument from the given initialization values.

        Parameters
        ----------
        values : dict[str, object]
            The values to initialize the instrument with.

        Returns
        -------
        CryptoPerpetual

        """
        return CryptoPerpetual.from_dict_c(values)

    @staticmethod
    def to_dict(CryptoPerpetual obj) -> dict[str, object]:
        """
        Return a dictionary representation of this object.

        Returns
        -------
        dict[str, object]

        """
        return CryptoPerpetual.to_dict_c(obj)

    @staticmethod
    cdef CryptoPerpetual from_pyo3_c(pyo3_instrument):
        return CryptoPerpetual(
            instrument_id=InstrumentId.from_str_c(pyo3_instrument.id.value),
            raw_symbol=Symbol(pyo3_instrument.raw_symbol.value),
            base_currency=Currency.from_str_c(pyo3_instrument.base_currency.code),
            quote_currency=Currency.from_str_c(pyo3_instrument.quote_currency.code),
            settlement_currency=Currency.from_str_c(pyo3_instrument.settlement_currency.code),
            is_inverse=pyo3_instrument.is_inverse,
            price_precision=pyo3_instrument.price_precision,
            size_precision=pyo3_instrument.size_precision,
            price_increment=Price.from_raw_c(pyo3_instrument.price_increment.raw, pyo3_instrument.price_precision),
            size_increment=Quantity.from_raw_c(pyo3_instrument.size_increment.raw, pyo3_instrument.size_precision),
            multiplier=Quantity.from_raw_c(pyo3_instrument.multiplier.raw, pyo3_instrument.multiplier.precision),
            max_quantity=Quantity.from_raw_c(pyo3_instrument.max_quantity.raw,pyo3_instrument.max_quantity.precision) if pyo3_instrument.max_quantity is not None else None,
            min_quantity=Quantity.from_raw_c(pyo3_instrument.min_quantity.raw,pyo3_instrument.min_quantity.precision) if pyo3_instrument.min_quantity is not None else None,
            max_notional=Money.from_str_c(str(pyo3_instrument.max_notional)) if pyo3_instrument.max_notional is not None else None,
            min_notional=Money.from_str_c(str(pyo3_instrument.min_notional)) if pyo3_instrument.min_notional is not None else None,
            max_price=Price.from_raw_c(pyo3_instrument.max_price.raw,pyo3_instrument.max_price.precision) if pyo3_instrument.max_price is not None else None,
            min_price=Price.from_raw_c(pyo3_instrument.min_price.raw,pyo3_instrument.min_price.precision) if pyo3_instrument.min_price is not None else None,
            margin_init=Decimal(pyo3_instrument.margin_init),
            margin_maint=Decimal(pyo3_instrument.margin_maint),
            maker_fee=Decimal(pyo3_instrument.maker_fee),
            taker_fee=Decimal(pyo3_instrument.taker_fee),
            ts_event=pyo3_instrument.ts_event,
            ts_init=pyo3_instrument.ts_init,
            info=pyo3_instrument.info,
        )

    @staticmethod
    def from_pyo3(pyo3_instrument):
        return CryptoPerpetual.from_pyo3_c(pyo3_instrument)
