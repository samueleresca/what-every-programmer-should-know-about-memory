# Bypass cache

`bypass_cache_perf.sh` execute `perf` on the code:
- `bypass_cache_perf.sh standard` initialize an array with the following length `1_000_000_000`. The array is initialized in a standard way.
- `bypass_cache_perf.sh nocache` initialize an array with the following length `1_000_000_000`.  The array is initialized using the *non-temporal* write operations.