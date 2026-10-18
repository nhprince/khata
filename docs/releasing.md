# Releasing Khata

Releases are automatic. `.github/workflows/release.yml` builds every installer and publishes the
GitHub Release; you only choose the version.

## One-time setup (already done if you followed the setup guide)

1. Push the repository to GitHub (the workflows live in `.github/workflows/`).
2. *Settings › Pages › Build and deployment › Source*: choose **GitHub Actions**. This publishes the
   in-browser version as a live demo at `https://<you>.github.io/khata/` on every release.
3. Nothing else: no secrets are needed. Signing is optional (below).

## Making a release

**One click (recommended).** *Actions › Release › Run workflow*, type the new version (for example
`0.2.0`) and run it. The workflow bumps the version in `Cargo.toml` and `Cargo.lock`, commits
`Release v0.2.0` to the branch, tags it, builds everything, and publishes the release with
checksums and a change list taken from your commit messages.

**By hand.** `cargo xtask version set 0.2.0`, commit, `git tag v0.2.0`, `git push --tags`. The tag
starts the same workflow; it refuses a tag that does not match the version in `Cargo.toml`.

**Dry run.** Run the workflow with the version box empty. Everything is built and attached to the
run (Actions › the run › Artifacts) but nothing is committed or published. Do this before the first
release and after changing packaging.

Pre-releases: a version with a suffix (`0.2.0-beta.1`) is published as a pre-release.

## What gets built

| Platform | Artifacts | Required |
|---|---|---|
| Windows | `.msi` installer (WiX v5) and a portable `.zip` | yes |
| Linux | `.AppImage`, `.deb`, `.rpm`, `.tar.gz` (built on Ubuntu 22.04, so they run on newer distros) | yes |
| macOS | universal `.dmg` and a CLI zip | no (best effort) |
| Web | `khata-web-<version>.zip` and the GitHub Pages live demo | no (best effort) |

A failure in an optional job never blocks the release; the release page just lacks that file. If
the MSI or the AppImage step fails, the workflow still ships the portable zip or the `.deb`/`.tar.gz`
and prints a warning in the run.

## Version numbers

The single source of truth is `[workspace.package] version` in the root `Cargo.toml`
(`cargo xtask version` prints it). The app shows it in `--version` and About.

## Code signing (optional, later)

Without signing, Windows SmartScreen shows "Unknown publisher" and macOS asks the user to
right-click › Open. The packaging scripts already support signing and skip it silently when
secrets are missing:

- **Windows:** `packaging/windows/sign.ps1` (Azure Trusted Signing; `AZURE_*` secrets).
- **macOS:** `MACOS_SIGN_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` (needs a paid Apple
  Developer account) with `packaging/macos/import-cert.sh`.

Add the secrets under *Settings › Secrets and variables › Actions* and pass them to the job's
`env:` in `release.yml`.

## Local packaging

`packaging/windows/package.ps1`, `packaging/linux/package.sh`, `packaging/macos/package.sh` and
`packaging/web/package.sh` produce the same artifacts in `dist/release/` on your own machine.
