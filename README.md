# Lightyear — maintained networking fork

Lightyear provides Bevy networking: transports and connections, buffered input,
replication, prediction/rollback and interpolation. It supports client/server,
host-client and P2P arrangements. Deterministic replication requires a deterministic
application simulation.

This BoysGameStudio fork targets the Bevy/Lightyear versions in its manifests
(currently Bevy 0.19.1 / Lightyear 0.29). Its input provenance, rebroadcast and
rollback guarantees are described in [fork contracts](docs/fork/CONTRACTS.md).
Upstream registry releases and hosted docs do not establish those fork guarantees.

## Start here

For a sibling checkout:

```toml
[dependencies]
lightyear = { path = "../lightyear/crates/core/lightyear" }
```

Select client/server, input and transport features for the application; the
[facade manifest](crates/core/lightyear/Cargo.toml) owns the feature names.
For a git dependency, select this fork and an explicit reviewed revision instead
of copying a registry-only version requirement. Keep dependent engine sources coherent.

Start with [example setup](examples/README.md) and
[simple_box](examples/simple_box/README.md). The [book](book/src/SUMMARY.md)
explains concepts and routes to this checkout's examples. Run commands from the
workspace root after [dependency preparation](LOCAL_VALIDATION.md).

## Documentation

- [Fork contracts](docs/fork/CONTRACTS.md): maintained semantics and host obligations.
- [Local validation](LOCAL_VALIDATION.md): selective checks and command side effects.
- [Allocation regression](docs/allocation-regression.md): workload and measurement scope.
- [Agent instructions](AGENTS.md).
- [Upstream project](https://github.com/cBournhonesque/lightyear) and
  [upstream API reference](https://docs.rs/lightyear): distinguish upstream from this fork.
- [MIT](LICENSE-MIT) / [Apache-2.0](LICENSE-APACHE) licenses.

## Repository layout

Workspace crate sources live under `crates/`, grouped by role. Directory names drop the `lightyear_` prefix, but Cargo package names keep it.

- `crates/io`: low-level IO links and backends such as `aeronet`, `link`, `udp`, `crossbeam`, `websocket`, and `webtransport`
- `crates/connection`: connection abstractions and adapters such as `connection`, `raw_connection`, `netcode`, and `steam`
- `crates/core`: the top-level `lightyear` crate plus shared core, sync, utils, and frame interpolation crates
- `crates/inputs`: input crates such as `inputs`, `inputs_native`, `input_bei`, and `inputs_leafwing`
- `crates/replication`: replication, prediction, and interpolation crates
- `crates/transport`: serialization, transport, and message crates
- `crates/integration`: Bevy ecosystem integrations such as Avian
- `crates/platform`, `crates/deterministic`, `crates/tools`, and `crates/tests`: platform support, deterministic replication, tooling, and test support
