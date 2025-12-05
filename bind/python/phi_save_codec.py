import json
import base64
from typing import Any
from wasmtime import Store, Module, Instance, Memory
import ctypes

class PhiSaveCodec:
    def __init__(self, wasm_path: str = "phi_save_codec.wasm"):
        self.store = Store()
        self.module = Module.from_file(self.store.engine, wasm_path)
        self.instance = Instance(self.store, self.module, [])

        self.parse_func = self.instance.exports(self.store)["parse_game_key"]
        self.build_func = self.instance.exports(self.store)["build_game_key"]
        self.malloc_func = self.instance.exports(self.store)["malloc"]
        self.free_func = self.instance.exports(self.store)["free"]
        self.memory: Memory = self.instance.exports(self.store)["memory"]

    def _write_string(self, data: bytes) -> tuple[int, int]:
        data0 = data + b"\0"
        size = len(data0)
        ptr = self.malloc_func(self.store, size)
        if ptr == 0:
            raise MemoryError("Failed to allocate memory in WASM")
        
        buf_ptr = self.memory.data_ptr(self.store)
        dest = ctypes.c_void_p(ctypes.addressof(buf_ptr.contents) + ptr)
        ctypes.memmove(dest, data0, size)
        return ptr, size

    def _read_cstring(self, ptr: int) -> str:
        if ptr == 0:
            return ""
            
        buf_ptr = self.memory.data_ptr(self.store)
        offset = 0
        while True:
            c = buf_ptr[ptr + offset]
            if c == 0:
                break
            offset += 1
        
        data = bytes(buf_ptr[ptr:ptr+offset])
        return data.decode("utf-8")

    def parse_game_key(self, data: bytes) -> dict[str, Any]:
        b64 = base64.b64encode(data)
        ptr, size = self._write_string(b64)
        
        out_ptr = self.parse_func(self.store, ptr)
        if out_ptr == 0:
            raise ValueError("Failed to parse game key")
        
        result = self._read_cstring(out_ptr)
        self.free_func(self.store, ptr, size)
        
        return json.loads(result)

    def build_game_key(self, game_key_dict: dict[str, Any]) -> bytes:
        json_str = json.dumps(game_key_dict, ensure_ascii=False).encode('utf-8')
        json_b64 = base64.b64encode(json_str)
        
        ptr, size = self._write_string(json_b64)
        
        out_ptr = self.build_func(self.store, ptr)
        if out_ptr == 0:
            raise ValueError("Failed to build game key")
        
        result_str = self._read_cstring(out_ptr)
        self.free_func(self.store, out_ptr, size)
        
        return base64.b64decode(result_str)