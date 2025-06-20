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

import pyarrow as pa

from nautilus_trader.core.data import Data
from nautilus_trader.serialization.arrow.serializer import register_arrow
from nautilus_trader.serialization.base import register_serializable_type


def generate_signal_class(name: str, value_type: type) -> type:
    """
    Dynamically create a Data subclass for this signal.

    Parameters
    ----------
    name : str
        The name of the signal data.
    value_type : type
        The type for the signal data value.

    Returns
    -------
    SignalData

    """

    class SignalData(Data):
        """
        Represents generic signal data.
        """

        def __init__(self, value: object, ts_event: int, ts_init: int) -> None:
            self.value = value
            self._ts_event = ts_event
            self._ts_init = ts_init

        @property
        def ts_event(self) -> int:
            """
            UNIX timestamp (nanoseconds) when the data event occurred.

            Returns
            -------
            int

            """
            return self._ts_event

        @property
        def ts_init(self) -> int:
            """
            UNIX timestamp (nanoseconds) when the object was initialized.

            Returns
            -------
            int

            """
            return self._ts_init

    SignalData.__name__ = f"Signal{name.title()}"

    # Dictionary serialization for message bus and Redis
    def to_dict_c(obj: SignalData) -> dict[str, object]:
        return {
            "type": type(obj).__name__,
            "value": obj.value,
            "ts_event": obj.ts_event,
            "ts_init": obj.ts_init,
        }

    def from_dict_c(values: dict[str, object]) -> SignalData:
        return SignalData(
            value=values["value"],
            ts_event=int(values["ts_event"]),  # type: ignore
            ts_init=int(values["ts_init"]),  # type: ignore
        )

    # Add serialization methods to the class
    SignalData.to_dict_c = to_dict_c
    SignalData.from_dict_c = from_dict_c
    SignalData.to_dict = lambda obj: SignalData.to_dict_c(obj)
    SignalData.from_dict = lambda values: SignalData.from_dict_c(values)

    # Parquet serialization
    def serialize_signal(data: SignalData) -> pa.RecordBatch:
        return pa.RecordBatch.from_pylist(
            [
                {
                    "ts_init": data.ts_init,
                    "ts_event": data.ts_event,
                    "value": data.value,
                },
            ],
            schema=schema,
        )

    def deserialize_signal(table: pa.Table) -> list[SignalData]:
        return [SignalData(**d) for d in table.to_pylist()]

    schema = pa.schema(
        {
            "ts_event": pa.uint64(),
            "ts_init": pa.uint64(),
            "value": {
                int: pa.int64(),
                float: pa.float64(),
                str: pa.string(),
                bool: pa.bool_(),
                bytes: pa.binary(),
            }[value_type],
        },
    )
    # Register for arrow serialization (only if not already registered)
    try:
        register_arrow(
            data_cls=SignalData,
            encoder=serialize_signal,
            decoder=deserialize_signal,
            schema=schema,
        )
    except (KeyError, ValueError):
        # Already registered, skip
        pass

    # Register for message bus serialization (only if not already registered)
    try:
        register_serializable_type(
            cls=SignalData,
            to_dict=SignalData.to_dict_c,
            from_dict=SignalData.from_dict_c,
        )
    except KeyError:
        # Already registered, skip
        pass

    return SignalData
