import { WASI } from 'node:wasi'
import fs from 'node:fs'
const wasi = new WASI({
  version: 'preview1',
  env: process.env,
})

const wasm = await WebAssembly.compile(
  new Uint8Array(fs.readFileSync('target/wasm32-wasip1-threads/debug/wasi_thread_local.wasm')),
)

const instance = await WebAssembly.instantiate(wasm, {
  ...wasi.getImportObject(),
  env: {
    memory: new WebAssembly.Memory({ initial: 1024, maximum: 65536, shared: true }),
  },
})

wasi.initialize(instance);

console.log(instance.exports.set_thread_local_data(1, 2))
