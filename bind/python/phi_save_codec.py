import json
from typing import Any
from wasmtime import Store, Module, Instance, Memory
import ctypes

class PhiSaveCodec:
    def __init__(self, wasm_path: str = "phi_save_codec.wasm"):
        self.store = Store()
        self.module = Module.from_file(self.store.engine, wasm_path)
        self.instance = Instance(self.store, self.module, [])
        self.memory: Memory = self.instance.exports(self.store)["memory"]
        self.malloc_func = self.instance.exports(self.store)["malloc"]
        self.free_func = self.instance.exports(self.store)["free"]

        self.func_list = ["game_key", "game_record"]

        for name in self.func_list:
            parse_func = self.instance.exports(self.store)[f"parse_{name}"]
            build_func = self.instance.exports(self.store)[f"build_{name}"]

            setattr(self, f"parse_{name}", self._make_parser(parse_func))
            setattr(self, f"build_{name}", self._make_builder(build_func))

    def _write(self, data: bytes, size: int) -> int:
        ptr = self.malloc_func(self.store, size)
        if ptr == 0:
            raise MemoryError("Failed to allocate memory in WASM")
        buf_ptr = self.memory.data_ptr(self.store)
        dest = ctypes.c_void_p(ctypes.addressof(buf_ptr.contents) + ptr)
        ctypes.memmove(dest, data, size)
        return ptr

    def _read(self, ptr: int, size: int) -> bytes:
        if ptr == 0:
            return b""
        buf_ptr = self.memory.data_ptr(self.store)
        data = bytes(buf_ptr[ptr:ptr+size])
        self.free_func(self.store, ptr, size)
        return data

    def _call_wasm(self, wasm_func, data: bytes) -> bytes:
        size = len(data)
        ptr = self._write(data, size)
        out_size, out_ptr = wasm_func(self.store, ptr, size)
        if out_ptr == 0 or out_size == 0:
            raise ValueError("WASM call returned 0 (error)")
        result = self._read(out_ptr, out_size)
        return result

    def _call_parser(self, wasm_func, data: bytes) -> dict[str, Any]:
        out = self._call_wasm(wasm_func, data)
        return json.loads(out)

    def _call_builder(self, wasm_func, data_dict: dict[str, Any]) -> bytes:
        json_str = json.dumps(data_dict, ensure_ascii=False).encode("utf-8")
        out = self._call_wasm(wasm_func, json_str)
        return out

    def _make_parser(self, wasm_func):
        def parse_func(data: bytes) -> dict[str, Any]:
            return self._call_parser(wasm_func, data)
        return parse_func

    def _make_builder(self, wasm_func):
        def build_func(data_dict: dict[str, Any]) -> bytes:
            return self._call_builder(wasm_func, data_dict)
        return build_func
    
    def build_game_key(data_dict: dict[str, Any]) -> bytes: ...
    
    def parse_game_key(data: bytes)-> dict[str, Any]: ...
    
    def build_game_record(data_dict: dict[str, Any]) -> bytes: ...
    
    def parse_game_record(data: bytes)-> dict[str, Any]: ...
