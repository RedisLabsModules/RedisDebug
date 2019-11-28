[![GitHub issues](https://img.shields.io/github/release/RedisLabsModules/RedisDebug.svg)](https://github.com/RedisLabsModules/RedisDebug/releases/latest)
[![CircleCI](https://circleci.com/gh/RedisLabsModules/RedisDebug/tree/master.svg?style=svg)](https://circleci.com/gh/RedisLabsModules/RedisDebug/tree/master)

# RedisDebug
Extension modules to Redis' Debug


# Getting Started

## Build

```bash
git clone https://github.com/RedisLabsModules/RedisDebug.git
cd RedisX
cargo build --release
```

## Run 
```
redis-server --loadmodule ./target/release/RedisDebug.so
```

# Commands

## DEBUG.FORK
## DEBUG.CORE
