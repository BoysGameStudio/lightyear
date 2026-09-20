# Local validation

Remote CI workflows and Actions-only components were removed on 2026-09-19
at the user’s request. Do not recreate, dispatch or require remote jobs.
The removed definitions remain available in Git history, not as active runners.

Run relevant commands from this project directory; these are selectable checks,
not a mandatory full batch for every change. Keep Cargo and GPU runs serialized.
Frozen resolution requires already prepared dependencies and a matching lockfile.

```sh
just --list
```

Use the existing README and domain runbooks for affected runtime, GPU, asset and
platform checks. A check on Linux does not qualify Windows, macOS or mobile.
Record the source revision, command, configuration, device where relevant, exit
status and evidence directory. Missing inputs are not passing results.

Deleting local workflow files does not change a remote branch until publication.
Actions were disabled and read back as disabled on all ten workbench repositories
on 2026-09-19. No queued or active runs were found. Recheck settings before any
authorized publication. Exact published revisions are recorded by the workbench.

## Safe selective tasks and fork ownership

Do not run bare `just` as an inspection command: its default recipe formats Rust
and TOML and invokes `typos -w` before broad checks. Keep fixes explicit. These
commands check the affected fork libraries without running that default recipe:

```sh
cargo fmt -p lightyear_inputs -- --check
CARGO_INCREMENTAL=0 cargo test --frozen -p lightyear_inputs --lib --features client,server
CARGO_INCREMENTAL=0 cargo test --frozen -p lightyear_prediction --lib --features deterministic
CARGO_INCREMENTAL=0 cargo check --frozen -p lightyear_deterministic_replication --lib --no-default-features --features std,client,replication
CARGO_INCREMENTAL=0 cargo check --frozen -p lightyear_deterministic_replication --lib --no-default-features --features std,server,replication
```

Run commands serially. The tests workspace's default Steam feature and broad
example/Avian feature graphs are separate acceptance scopes, not prerequisites for
these checks. The [fork ledger](docs/fork/CONTRACTS.md) owns semantic patch,
retirement and pacing obligations; a successful check does not replace a real
network fleet. For a fresh checkout without the locally ignored lock, explicitly
prepare dependencies with `cargo generate-lockfile` then `cargo fetch --locked`,
record that lock and toolchain, and only then use frozen checks. Preparation may
resolve dependencies and must not be represented as the old qualified lock.
