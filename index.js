/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'scylladb.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.android-arm64.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'scylladb.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.android-arm-eabi.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'scylladb.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'scylladb.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'scylladb.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'scylladb.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./scylladb.darwin-universal.node')
      } else {
        nativeBinding = require('@lambda-group/scylladb-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'scylladb.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.darwin-x64.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'scylladb.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.darwin-arm64.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'scylladb.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./scylladb.freebsd-x64.node')
      } else {
        nativeBinding = require('@lambda-group/scylladb-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-x64-musl.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'scylladb.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./scylladb.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@lambda-group/scylladb-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'scylladb.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./scylladb.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@lambda-group/scylladb-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { Compression, Consistency, SerialConsistency, Cluster, VerifyMode, BatchStatement, PreparedStatement, Query, Metrics, ScyllaSession, ClusterDataSimplified, KeyspaceSimplified, TableSimplified, MaterializedViewSimplified, Uuid } = nativeBinding

module.exports.Compression = Compression
module.exports.Consistency = Consistency
module.exports.SerialConsistency = SerialConsistency
module.exports.Cluster = Cluster
module.exports.VerifyMode = VerifyMode
module.exports.BatchStatement = BatchStatement
module.exports.PreparedStatement = PreparedStatement
module.exports.Query = Query
module.exports.Metrics = Metrics
module.exports.ScyllaSession = ScyllaSession
module.exports.ClusterDataSimplified = ClusterDataSimplified
module.exports.KeyspaceSimplified = KeyspaceSimplified
module.exports.TableSimplified = TableSimplified
module.exports.MaterializedViewSimplified = MaterializedViewSimplified
module.exports.Uuid = Uuid

const customInspectSymbol = Symbol.for('nodejs.util.inspect.custom')

Uuid.prototype[customInspectSymbol] = function () {
  return this.toString();
}