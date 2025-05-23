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

from cpython.datetime cimport datetime
from libc.stdint cimport uint64_t

from nautilus_trader.backtest.exchange cimport SimulatedExchange
from nautilus_trader.common.component cimport Clock
from nautilus_trader.common.component cimport Logger
from nautilus_trader.core.data cimport Data
from nautilus_trader.core.rust.backtest cimport TimeEventAccumulatorAPI
from nautilus_trader.core.rust.core cimport CVec
from nautilus_trader.core.uuid cimport UUID4
from nautilus_trader.data.engine cimport DataEngine
from nautilus_trader.data.messages cimport DataCommand
from nautilus_trader.data.messages cimport DataResponse
from nautilus_trader.data.messages cimport RequestData
from nautilus_trader.data.messages cimport SubscribeData
from nautilus_trader.data.messages cimport UnsubscribeData
from nautilus_trader.model.data cimport Bar
from nautilus_trader.model.data cimport QuoteTick
from nautilus_trader.model.data cimport TradeTick
from nautilus_trader.model.identifiers cimport InstrumentId
from nautilus_trader.model.identifiers cimport Venue


cdef class BacktestEngine:
    cdef object _config
    cdef Clock _clock
    cdef Logger _log
    cdef TimeEventAccumulatorAPI _accumulator

    cdef object _kernel
    cdef UUID4 _instance_id
    cdef DataEngine _data_engine
    cdef str _run_config_id
    cdef UUID4 _run_id
    cdef datetime _run_started
    cdef datetime _run_finished
    cdef datetime _backtest_start
    cdef datetime _backtest_end

    cdef dict[Venue, SimulatedExchange] _venues
    cdef set[InstrumentId] _has_data
    cdef set[InstrumentId] _has_book_data
    cdef list[Data] _data
    cdef uint64_t _data_len
    cdef uint64_t _index
    cdef uint64_t _iteration
    cdef object _data_iterator
    cdef uint64_t _last_ns
    cdef uint64_t _end_ns
    cdef dict[str, RequestData] _data_requests
    cdef set[str] _backtest_subscription_names

    cdef CVec _advance_time(self, uint64_t ts_now)
    cdef void _process_raw_time_event_handlers(
        self,
        CVec raw_handlers,
        uint64_t ts_now,
        bint only_now,
        bint as_of_now=*,
    )

    cpdef void _handle_data_command(self, DataCommand command)
    cpdef void _handle_subscribe(self, SubscribeData command)
    cpdef void _handle_data_response(self, DataResponse response)
    cpdef void _handle_unsubscribe(self, UnsubscribeData command)
    cpdef void _handle_empty_data(self, str subscription_name, uint64_t last_ts_init)


cdef inline bint should_skip_time_event(
    uint64_t ts_event_init,
    uint64_t ts_now,
    bint only_now,
    bint as_of_now,
):
    if only_now and ts_event_init < ts_now:
        return True
    if (not only_now) and (ts_event_init == ts_now):
        return True
    if as_of_now and ts_event_init > ts_now:
        return True

    return False


cdef class BacktestDataIterator:
    cdef object _empty_data_callback
    cdef Logger _log
    cdef dict[str, list[Data]] _data
    cdef dict[str, str] _data_name
    cdef dict[str, str] _data_priority
    cdef dict[str, int] _data_len
    cdef dict[str, int] _data_index
    cdef list[tuple[uint64_t, str, int]] _heap
    cdef int _next_data_priority
    cdef list[Data] _single_data
    cdef str _single_data_name
    cdef int _single_data_priority
    cdef int _single_data_len
    cdef int _single_data_index
    cdef bint _is_single_data

    cpdef void _reset_single_data(self)
    cpdef void add_data(self, str data_name, list data_list, bint append_data=*)
    cdef void _add_data(self, str data_name, list data_list, bint append_data=*)
    cpdef void remove_data(self, str data_name)
    cpdef void _activate_single_data(self)
    cpdef void _deactivate_single_data(self)
    cpdef Data next(self)
    cpdef void _push_data(self, int data_priority, int data_index)
    cpdef void reset(self)
    cpdef void _reset_heap(self)
    cpdef void set_index(self, str data_name, int index)
    cpdef bint is_done(self)
    cpdef dict all_data(self)
    cpdef list data(self, str data_name)
