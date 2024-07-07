Run this a few times:
```
touch src/lib.rs && RUSTFLAGS="-Zthreads=0 -Zmir-enable-passes=+GVN,+Inline -Zmir-opt-level=1" cargo build
```
That should produce this ICE:
```
thread 'rustc' panicked at /rustc/cc8da78a036dc3c15c35a97651b02af9a6d30c1e/compiler/rustc_data_structures/src/sync.rs:337:42:
assertion failed: *old == value
...

note: rustc 1.81.0-nightly (cc8da78a0 2024-07-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED] -C opt-level=2 -Z threads=0

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `fastrand::seed`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
```
