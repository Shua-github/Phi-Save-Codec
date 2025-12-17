import msgpack
from typing import Any, Callable
from wasmtime import Store, Module, Instance
import ctypes

class Codec:
    @staticmethod
    def loads(data: bytes) -> Any:
        return msgpack.unpackb(data)

    @staticmethod
    def dumps(obj: Any) -> bytes:
        return msgpack.packb(obj)

class _WasmParser:
    def __init__(self, name: str):
        self.name = name
        self._func: Callable[[bytes], dict[str, Any]] | None = None

    def __get__(self, obj:"PhiSaveCodec", objtype=None) -> Callable[[bytes], dict[str, Any]]:
        if self._func is None:
            wasm_func = obj._exports[f"parse_{self.name}"]

            def parse(data: bytes) -> dict[str, Any]:
                return obj._call_parser(wasm_func, data)

            self._func = parse

        return self._func


class _WasmBuilder:
    def __init__(self, name: str):
        self.name = name
        self._func: Callable[[dict[str, Any]], bytes] | None = None

    def __get__(self, obj:"PhiSaveCodec", objtype=None) -> Callable[[dict[str, Any]], bytes]:
        if self._func is None:
            wasm_func = obj._exports[f"build_{self.name}"]

            def build(data: dict[str, Any]) -> bytes:
                return obj._call_builder(wasm_func, data)

            self._func = build

        return self._func

class PhiSaveCodec:
    parse_game_key: Callable[[bytes], dict[str, Any]] = _WasmParser("game_key")
    build_game_key: Callable[[dict[str, Any]], bytes] = _WasmBuilder("game_key")

    parse_game_record: Callable[[bytes], dict[str, Any]] = _WasmParser("game_record")
    build_game_record: Callable[[dict[str, Any]], bytes] = _WasmBuilder("game_record")

    parse_game_progress: Callable[[bytes], dict[str, Any]] = _WasmParser("game_progress")
    build_game_progress: Callable[[dict[str, Any]], bytes] = _WasmBuilder("game_progress")

    parse_user: Callable[[bytes], dict[str, Any]] = _WasmParser("user")
    build_user: Callable[[dict[str, Any]], bytes] = _WasmBuilder("user")

    parse_summary: Callable[[bytes], dict[str, Any]] = _WasmParser("summary")
    build_summary: Callable[[dict[str, Any]], bytes] = _WasmBuilder("summary")

    def __init__(self, wasm_path: str = "phi_save_codec.wasm"):
        self._store = Store()
        self._module = Module.from_file(self._store.engine, wasm_path)
        self._instance = Instance(self._store, self._module, [])
        self._exports = self._instance.exports(self._store)

    def _malloc(self, size: int) -> int:
        return self._exports["malloc"](self._store, size)

    def _free(self, ptr: int, size: int) -> None:
        self._exports["free"](self._store, ptr, size)

    def _write(self, data: bytes, size: int) -> int:
        ptr = self._malloc(size)
        if ptr == 0:
            raise MemoryError("Failed to allocate memory in WASM")

        mem_ptr = self._exports["memory"].data_ptr(self._store)
        dest = ctypes.c_void_p(ctypes.addressof(mem_ptr.contents) + ptr)
        ctypes.memmove(dest, data, size)
        return ptr

    def _read(self, ptr: int, size: int) -> bytes:
        if ptr == 0 or size == 0:
            return b""
        mem = self._exports["memory"].data_ptr(self._store)
        return bytes(mem[ptr : ptr + size])

    def _call_wasm(self, wasm_func, data: bytes) -> bytes:
        size = len(data)
        ptr = self._write(data, size)

        out_size, out_ptr = wasm_func(self._store, ptr, size)
        if out_ptr == 0 or out_size == 0:
            self._free(ptr, size)
            raise ValueError("WASM call returned error")

        result = self._read(out_ptr, out_size)

        self._free(ptr, size)
        self._free(out_ptr, out_size)
        return result

    def _call_parser(self, wasm_func, data: bytes) -> dict[str, Any]:
        out = self._call_wasm(wasm_func, data)
        return Codec.loads(out)

    def _call_builder(self, wasm_func, data_dict: dict[str, Any]) -> bytes:
        data = Codec.dumps(data_dict)
        return self._call_wasm(wasm_func, data)
