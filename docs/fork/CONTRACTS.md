# Maintained fork contracts

This ledger describes the checked-out 0.29 fork's final mechanisms, not the order
of historical fixup/revert commits. Upstream issue states below are historical
consumer evidence, not newly verified upstream status. Retire a patch only after
the replacement version passes its decisive regression and affected host gate.
No remote CI is part of that process.

| Contract / trigger | Owning source | Regression / acceptance | Consumer obligation and retirement condition |
|---|---|---|---|
| A gap fill is not received input, even after popping its predecessor | `crates/inputs/inputs/src/input_buffer.rs` | `test_front_fill_keeps_fabricated_marker_after_pop`, server-pop prediction fallback tests | Coverage pacing must use the raw received/fabricated distinction. Retire only if upstream preserves both resolution and provenance. |
| An equal-valued newly received input can repair fabricated coverage | `crates/inputs/inputs_leafwing/src/input_message.rs` | `test_update_buffer_corrects_mismatches_at_confirmed_ticks`, equal-content tests; consumer covered-tick reproduction | Do not discard a real input merely because its value equals a fill. The consumer recorded immutable upstream `503febdbd45a56c429f26b963c611ed427548916` RED and fork `f68f0adc02ce2881df34bc31b8e7bd5ca8c0d8c7` GREEN; no new upstream status is asserted here. |
| A lost rebroadcast must remain repairable from retained server input | `crates/inputs/inputs/src/server.rs`, exported `SERVER_REBROADCAST_HISTORY_DEPTH` in `lib.rs` | Input-window/lookahead tests and consumer packet-loss fleet | Rebuild outgoing windows from the server's buffer. Reference consumer rollback horizon + 1 must fit the exported 101-tick window. Do not shorten it independently of pacing/retention. |
| Forced rollback restore kind and target are separate | `crates/replication/prediction/src/manager.rs`, `rollback.rs` | `equal_tick_retains_the_first_restore_kind_and_earlier_tick_replaces_it`, rollback history tests | Late input events use FromInputs; state catch-up uses FromState. Earlier targets replace later ones; equal targets retain the first kind. Do not change tie policy through code movement. |
| Empty/history-miss restores must not leave a future component value | `crates/replication/prediction/src/rollback.rs`, deterministic checksum | `test_predicted_component_initial_rollback`, `test_rollback_preserves_pre_target_history_for_deeper_rollback` | Preserve pre-target history and confirmed seed fallback. Do not reintroduce game-side history refresh workarounds.|
| Mid-session clock shifts must not skip locally produced inputs | `crates/core/sync/src/timeline/input.rs`, `timeline/sync.rs` | Consumer exact-input coverage/late-join/reconnect gates | Initial synchronization and running-session pacing are different phases. Running deterministic peers slew rather than relabel an unproduced input gap. Preserve traces until equivalent observation exists. |
| Rollback bypassing Leafwing's outer fixed swap must preserve fixed state | `crates/inputs/inputs_leafwing/src/plugin.rs` | Leafwing input/rollback regressions and consumer pulse-bearing fleet | `sync_fixed_update_state_after_rollback` runs after the rollback input tick. Retirement requires replayed edge semantics, not just matching serialization. |
| P2P checksums can arrive before or after their local counterpart | `crates/deterministic/deterministic_replication/src/checksum.rs` | `remote_checksum_waits_for_the_local_checksum`, `local_checksum_waits_for_each_remote_peer` | Retain pending comparisons and report divergence; a missing local sample is not proof of a match. |

## Clock and pacing ownership

The default deterministic prediction-window controller is application-global and
uses confirmed remote-input coverage plus the configured rollback horizon. Its
client and P2P tests cover missing streams and effective depth. It does not replace
all application/server pacing arrangements.

The reference game deliberately installs `PredictionWindowControl { enabled: false }`
because it owns exact-input production/server catch-up pacing. Re-enabling both
controllers can stop input production while each side waits for the other. Keep
that explicit opt-out until the whole ownership model is replaced and the client,
server, relay, late-join, reconnect and P2P cases are requalified. A local unit-test
pass is not evidence that those network fleets ran.

## Source and role validation

See [local validation](../../LOCAL_VALIDATION.md). Preserve role-specific checks;
do not enable Steam or every example feature merely to validate these libraries.
The host's compile-time horizon assertion intentionally consumes the exported fork
constant. Publishing that host change requires publishing this fork revision first
and updating the host's dependency lock; a local workbench path override is not a
published registry/git release. Keep source provenance and role features separate.

## Replicon bridge and confirmed history

`crates/replication/replication/src/{client,server,channels}.rs` bridge Lightyear
transport and Replicon messages. `RepliconChannelMap` keeps separate server/client
channel namespaces: index zero in one namespace is not index zero in the other.
The current bridge supports bidirectional channel mapping; channel registration
in `channels.rs` owns ordering/reliability and direction.

`send.rs` owns replication, prediction and interpolation targets through Replicon
visibility. Prediction registration writes confirmed component history and detects
mismatches; `rollback.rs` owns restoring that history. Fully received mutation ticks
and per-entity explicit confirmation are different observations. An entity omitted
from a fully received tick retains its last confirmed value under the protocol's
acknowledgement/resend rules; that does not make an incomplete tick confirmed.
Keep later confirmed values while discarding/resimulating invalid predictions.

Prespawns use Replicon's `Signature` to map the server entity to an existing local
candidate before updates. `PreSpawnedReceiver` retains timeout/synchronization/
rollback bookkeeping; `PreSpawned::for_client` scopes mapping messages, not visibility.

Current prediction and replication regression sources under `crates/tests/src/`
own executable coverage. The old integration report's passing/ignored counts and
speculative failure causes are not current results. Inspect current `#[ignore]`
reasons when selecting a gate; static documentation work does not rerun the suite.
Unresolved behavior requires a current reproduction, not restoration of an old TODO.
