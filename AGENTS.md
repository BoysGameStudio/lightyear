# Agent instructions

Read README.md for project context and docs/fork/CONTRACTS.md and LOCAL_VALIDATION.md for the affected task.

- Preserve received/fabricated input provenance, retained rebroadcast history and
  rollback restore-kind ordering. A host horizon must fit the exported history window.
- Keep input-production pacing and prediction-window control under their documented
  owner; do not enable two competing controllers to fix a local symptom.
- Validate affected client/server feature graphs separately. Steam, all examples
  and live network fleets are separate scopes, not mandatory library checks.
- Bare `just` changes files before checking them. Use explicit inspection/check
  commands; keep local lock preparation and publication source identity explicit.
- Use local validation only; do not create, enable, dispatch or require remote CI.
- Choose checks for the affected behavior. Documentation-only edits use link,
  API/command-reference and packaging-input checks, without Cargo/GPU runs.
- Serialize Cargo/GPU work and set `CARGO_INCREMENTAL=0`. Report missing inputs
  and unexecuted platform checks; do not weaken tests or replace golden images.
- Preserve unrelated work and authoring inputs. Commit or publish only when asked.
