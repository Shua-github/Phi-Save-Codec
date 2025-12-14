import msgpack
from typing import Any
from wasmtime import Store, Module, Instance
import ctypes

FUNCS = ["game_key", "game_record", "user"]
class Codec:
    @staticmethod
    def loads(data: bytes) -> Any:
        return msgpack.unpackb(data)
    
    @staticmethod
    def dumps(obj: Any) -> bytes:
        return msgpack.packb(obj)

class PhiSaveCodec:
    def __init__(self, wasm_path: str = "phi_save_codec.wasm",funcs:list[str] | None = None):
        if funcs is None:
            funcs = FUNCS
            
        self._store = Store()
        self._module = Module.from_file(self._store.engine, wasm_path)
        self._instance = Instance(self._store, self._module, [])
        self._exports = self._instance.exports(self._store)

        for name in funcs:
            parse_func = self._instance.exports(self._store)[f"parse_{name}"]
            build_func = self._instance.exports(self._store)[f"build_{name}"]

            setattr(self, f"parse_{name}", self._make_parser(parse_func))
            setattr(self, f"build_{name}", self._make_builder(build_func))

    def _malloc(self, size:int) -> int:
        return self._exports["malloc"](self._store,size)
    
    def _free(self, ptr:int, size:int) -> int:
        return self._exports["free"](self._store,ptr,size)

    def _write(self, data: bytes, size: int) -> int:
        ptr = self._malloc(size)
        if ptr == 0:
            raise MemoryError("Failed to allocate memory in WASM")
        buf_ptr = self._exports["memory"].data_ptr(self._store)
        dest = ctypes.c_void_p(ctypes.addressof(buf_ptr.contents) + ptr)
        ctypes.memmove(dest, data, size)
        return ptr

    def _read(self, ptr: int, size: int) -> bytes:
        if ptr == 0:
            return b""
        buf_ptr = self._exports["memory"].data_ptr(self._store)
        data = bytes(buf_ptr[ptr:ptr+size])
        return data

    def _call_wasm(self, wasm_func, data: bytes) -> bytes:
        size = len(data)
        ptr = self._write(data, size)
        out_size, out_ptr = wasm_func(self._store, ptr, size)
        if out_ptr == 0 or out_size == 0:
            raise ValueError("WASM call returned 0 (error)")
        result = self._read(out_ptr, out_size)
        self._free(ptr,size)
        self._free(out_ptr,out_size)
        return result

    def _call_parser(self, wasm_func, data: bytes) -> dict[str, Any]:
        out = self._call_wasm(wasm_func, data)
        return Codec.loads(out)

    def _call_builder(self, wasm_func, data_dict: dict[str, Any]) -> bytes:
        data = Codec.dumps(data_dict)
        out = self._call_wasm(wasm_func, data)
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
    
    def build_user(data_dict: dict[str, Any]) -> bytes: ...
    
    def parse_user(data: bytes)-> dict[str, Any]: ...
