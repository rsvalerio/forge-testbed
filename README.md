# forge-testbed

Acceptance-test repository for [rsvalerio/forge](https://github.com/rsvalerio/forge).

It consumes **every** capability forge offers, so each one is exercised end to end before a
real project adopts it. It is simultaneously the acceptance-test suite, the integration
environment, and a worked example of the consumption pattern.

## It floats on `main` — deliberately the inverse of real consumers

Real repos pin a release tag, so one bad forge commit cannot break their pipeline. Every
wrapper here pins `@main`, and passes `forge-ref: main` so the composite actions load from
`main` too — otherwise a workflow from `main` would silently run against `v1`'s actions and
the testbed would be testing a mixture.

```
green testbed on main  →  tag vX.Y  →  real consumers pick it up
```

## Why it exists

Three forge bugs reached `rsvalerio/ops`'s release pipeline and broke it, one after another:

| Bug | Symptom | Would this repo have caught it? |
|---|---|---|
| `api-commit.sh` never committed | `No such file` (exit 127) | yes — first bump |
| `${{ }}` inside an action's `description` | `Unrecognized named-value: 'github'` | yes — first bump, at the first step |
| base64 through argv vs the 128 KiB cap | `jq: Argument list too long` (exit 126) | only once CHANGELOG.md grows past ~96 KiB |

Each was a **first-execution** failure, not a regression — nothing had ever run the code.
The third is the interesting one: it needs an accumulated changelog to trigger, so a
freshly-created testbed would have missed it too. Age is part of the fixture.

## Capability matrix

| Capability | How it is proven here | Real side effect |
|---|---|---|
| `rust-ci` | fmt, check, clippy, build, test, deny over the workspace | none |
| `mint-app-token` | token minted scoped to this repo only | none |
| `app-bot-identity` | bump commit authored by the App bot, not a human | none |
| `signed-commit` | the action asserts `verified=true` and fails otherwise | commit here only |
| `bump` | conventional commit → version, CHANGELOG, tag, dispatch | real tag here |
| `release` (dist) | one target, dispatched by bump | release here only |
| `publish-homebrew` | formula rendered and committed, **dry-run** | scratch tap |
| `publish-deb` | `.deb` built and staged, **dry-run** | scratch apt repo |
| `publish-crates` | `cargo publish --workspace --dry-run` | **never publishes** |

`publish-crates` stays dry-run permanently. crates.io publication is irreversible, so
`allow-real-publish` is absent rather than false — forge requires both interlocks, so no
single edit here can reach the registry.

## Layout

Multi-crate on purpose. A single-crate fixture silently skips publish ordering and the
`path` + `version` dependency rule that crates.io enforces.

```
crates/testbed-core/   no internal deps        — publishes first
crates/testbed-util/   depends on core         — proves ordering
crates/testbed-cli/    binary                  — what dist, homebrew and deb package
packaging/Makefile     minimal dpkg-deb build  — matches publish-deb's default layout
```

## Scratch targets

`publish-homebrew` and `publish-deb` check their destination out **even under dry-run**, so
both must exist:

- `rsvalerio/homebrew-tap-testbed`
- `rsvalerio/apt-testbed`

Never the production `rsvalerio/homebrew-tap` or `rsvalerio/apt`.
