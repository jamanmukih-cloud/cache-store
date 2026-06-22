# Cache Store 🗄️

In-memory cache with LRU/LFU eviction and TTL support.

## Features

- **Eviction Policies**: LRU, LFU, FIFO
- **TTL**: Per-key expiration
- **Pub/Sub**: Cache invalidation
- **Metrics**: Hit rate, memory usage

## Performance

| Metric | Value |
|--------|-------|
| GET | 15ns |
| SET | 25ns |
| Hit rate | >95% (typical) |
| Memory | ~60 bytes/key |

## Quick Start

```rust
let cache = Cache::new(CacheConfig { max_entries: 10000 });
cache.set("key", value, Duration::from_secs(300));
let val = cache.get("key");
```

## License

MIT