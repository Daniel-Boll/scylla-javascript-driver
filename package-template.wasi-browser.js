import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'
import { Volume as __Volume, createFsFromVolume as __createFsFromVolume } from '@napi-rs/wasm-runtime/fs'

import __wasmUrl from './package-template.wasm32-wasi.wasm?url'

const __fs = __createFsFromVolume(
  __Volume.fromJSON({
    '/': null,
  }),
)

const __wasi = new __WASI({
  version: 'preview1',
  fs: __fs,
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  // 1Gb
  initial: 16384,
  // 4Gb
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    return new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  },
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__plus_100_0']?.()
}
export const plus100 = __napiModule.exports.plus100
