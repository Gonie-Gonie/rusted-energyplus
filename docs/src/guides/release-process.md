---
status: active
claim_level: none
owner: release
last_reviewed: 2026-11-20
---

# Release process

The 0.2.0 release packages the accepted Source212 algorithm baseline. It does
not reopen object/parameter porting or generate new numerical compatibility
claims. Keep unfinished experiments outside the release checkout.

## Verify locally

Use the pinned Rust toolchain and locked EnergyPlus 26.1.0 runtime. Run the
workspace regression suite once, then the portable release gate:

```powershell
cargo fmt --all -- --check
cargo test --locked --workspace --lib --bins --tests
.\scripts\dev.cmd v0.2-verify
```

The gate checks generated docs, specifications and claim boundaries, builds
the locked release binary and launcher, and creates a curated ZIP. It verifies
the ZIP's inventory/checksums and extracts it outside the checkout. The packaged
CLI must report the expected version, show help, execute the unchanged A-24H
sample with its included converter/weather, and reject the blocked example.
Developer tool directories are removed from the smoke process's PATH.

The package uses an explicit file allowlist and native dependency closure.
It includes user documentation, support boundaries, example input/weather,
third-party notices, a manifest and file checksums. Development scripts,
scratch packets, `.runtime/`, `.reference/`, SDK examples and caches are excluded.

The old `v0.1-verify`/PDF evidence pipeline remains historical. It is not the
0.2.0 delivery gate and must not be substituted for package verification.

## Commit and publish

Update the workspace version and lockfile together, publish accurate release
notes, and commit/push validated changes. A `v0.2.0` tag triggers
`.github/workflows/release.yml`; the workflow repeats the checks in a fresh
Windows runner and publishes a prerelease only after package verification.

Public assets are the Windows ZIP, its SHA256 checksum and the verification
JSON bound to that exact ZIP. The full manifest and per-file `SHA256SUMS` are
inside the ZIP. Retained scientific evidence is described separately in
[current status](../current/current-status.md); historical local paths are not
silently packaged or treated as fresh verification.

When publishing manually, upload the same checked assets and use the matching
tag and release notes. Existing public versions and assets are not deleted as
part of preparing a new version.
