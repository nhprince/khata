# Khata rebrand: what changed

Khata is a fork of [storytold/wordcraft](https://github.com/storytold/wordcraft). This file lists every
change made in the rebrand, what was verified and what still needs a human with a real machine.

## 1. Identity

| Before | After |
|---|---|
| WordCraft | Khata (খাতা) |
| crates `wordcraft-*`, binaries `wordcraft`, `wordcraft-cli` | `khata-*`, `khata`, `khata-cli` |
| env vars `WORDCRAFT_*` | `KHATA_*` |
| app ID `ai.storyteller.wordcraft` | `me.nhprince.khata` |
| installer publisher "Learning Machines LLC" | NH Prince Pradhan |
| `github.com/storytold/wordcraft` | `github.com/nhprince/khata` (change this if your repo lives elsewhere) |

The rename was mechanical (412 files rewritten, 23 files/folders renamed) and the workspace
type-checks with it. **Left untouched on purpose:** `LICENSE-MIT`, `LICENSE-APACHE`, `contributors/`,
and the upstream copyright lines, which are accurate attribution. `NOTICE` and `ATTRIBUTION.md` were
extended, not rewritten. The `craft-libs` git dependencies stay: they are external libraries.

**Removed:** `docs/brand/` (the ArtCraft logos are trademarks and not part of the open-source
license), the old WordCraft screenshots in `docs/images/`, every Discord link and the Discord pill,
and the four CI workflows that call the upstream org's private shared actions.

## 2. Logo and icons

`assets/app-icon/khata.svg`: a coral spiral notebook with washi tape and the Bengali letter **খ**
(outline taken from the Atma Bold font, OFL). Generated from it: PNG sizes, Linux hicolor set, Windows
`.ico`, macOS `.icns`, a transparent `khata-mark.svg`, and the installer art.

## 3. Scrapbook theme (`crates/ui-egui/src/theme.rs`)

- New light palette (cream paper, kraft canvas, marker yellow, coral/chili accent, ink outlines) and
  a dark "night notebook" palette. New tokens: `ink`, `sun`, `tape`. The existing contrast tests still
  describe the rules: white on the accent is 4.9:1, input outlines are above 3:1.
- Fonts: Patrick Hand (interface), Kalam Bold (emphasis), Caveat Brush (display), Atma (Bengali).
  Inter stays behind them for scripts they lack. Sizes are scaled by `HAND_SCALE` (1.15) because
  handwriting faces have a small x-height.
- Widget styling: 2 px ink window and menu outlines, hard (blur-free) offset shadows, rounder corners.
- Helpers: `sticker`, `tape`, `squiggle`, `dot_grid`, `cover`.
- Applied to: the sticky-note active ribbon tab, the outlined title bar, search box, avatar, Share
  button and status bar, a dotted-paper canvas with sticker-style pages (ink outline plus yellow
  offset shadow), and the Backstage (yellow notebook-cover sidebar with a paper-sticker active item,
  hand-lettered headings with a marker squiggle, taped polaroid template cards, a sticky-note
  "Recent" empty state).
- The web build's loading screen and the About dialog (with খাতা and the upstream credit).

## 4. Insert › Shapes gallery (new)

- `crates/doc/src/shape_presets.rs`: about 70 shapes drawn from plain geometry, in Word's category
  layout: Rectangles, Basic Shapes, Block Arrows, Equation Shapes, Flowchart, Stars and Banners,
  Callouts. Native engine kinds stay native; the rest are inserted as freeform shapes, so they
  resize, print and export like any other shape.
- `crates/ui-egui/src/shapes_gallery.rs`: the menu. Thumbnails are painted from the same geometry,
  with a Recently Used row, the connector lines, and New Drawing Canvas.
- `insert.shape` accepts `{"preset": "<id>"}`. New shapes default to a marker-yellow fill with an ink
  outline (they were teal).
- The thumbnails are original drawings. No Microsoft artwork was copied (see the asset policy in
  `ATTRIBUTION.md`).

## 5. Usability

- **Simple mode:** Draw, Mailings, Developer and Zotero tabs are hidden until *Options › General ›
  Show advanced tabs* is ticked.
- Insert › Text icons that had no label (Signature Line, Date & Time, Object) now have one.
- Backstage navigation text for disabled items is easier to read, and small ribbon buttons are 25 px
  tall instead of 22 (easier to hit).

## 6. Packaging

- Windows installer: **new `UpgradeCode` and component GUIDs** (the old ones belonged to upstream
  WordCraft and would make the two products replace each other), file-type descriptions no longer say
  "Word", and the wizard uses scrapbook artwork (`installer-dialog.bmp`, `installer-banner.bmp`,
  regenerate with `packaging/windows/make-installer-art.py`). The MSI's *uninstall* experience is
  Windows' own and cannot be themed.
- Linux metainfo, `.deb`/`.rpm` metadata, web shell and the new `ci.yml` workflow.

## 7. Verification

Done in the sandbox (Rust 1.91 with three small, temporary compatibility patches to third-party
crates that need newer std APIs; those patches are **not** in this repository):

- `cargo check -p khata-ui-egui`: passes (this builds doc, engine, render, docx and the whole UI).
- `cargo check -p khata -p khata-cli -p xtask`: passes (desktop app, CLI, build tooling).
- `cargo test -p khata-doc shape_presets`: 3 of 3 pass. `cargo test -p khata-engine --lib`: 618 of 618.
- `cargo xtask assets`: passes (all 52 asset files are attributed in `ATTRIBUTION.md`).
- `cargo fmt --all`: applied once. The rename shortened identifiers (`wordcraft_` to `khata_`), which
  changes where rustfmt wraps lines in about 450 files. It was run with rustfmt 1.91; if CI's 1.95
  formats a few lines differently, run `cargo fmt --all` once more.
- `cargo test -p khata-ui-egui --lib`: 551 tests. The first run had 6 failures, all caused by this
  rebrand. After the fixes below, 550 of 551 passed; the last one (mini toolbar) was fixed by raising
  its row height from 27 to 28 px, and its 15 tests were re-run and pass. The *whole* suite was not
  re-run after that final one-line change, so run it once to confirm.

What the failures taught, and what changed because of them:

- Handwriting faces have a taller line (about 1.36 em, Inter is 1.2), so the ribbon grew:
  `CONTENT_H` 66 → 78, `LABEL_H` 18 → 20, big-button label pitch 13 → 16, small buttons 22 → 25 px
  (three stack to fill a group), and the mini toolbar's rows 24 → 28 px.
- Three tests encoded assumptions about the old blue/grey theme and were updated, not weakened:
  the hover test now accepts any colour channel moving (the cream hover colour leaves red at 255),
  the equation-picture test checks each channel between tile and text instead of requiring grey, and
  the Options-page test anchors on another checkbox since the Discord one is gone.
- The translation audit shows the advanced tabs, and lists Khata's new English-only strings as
  pending (`KHATA_PENDING` in `i18n/audit.rs`).

Not possible here, please do on your machine (Rust 1.95+):

1. `cargo check --workspace --all-targets` and `cargo xtask ci`.
2. `cargo run --release -p khata -- --sample` and look at it: the theme has never been seen on a real
   display. Expect to tune spacing; the font scale and the ribbon heights are the first suspects.
3. Open Insert › Shapes, insert a few presets, save as `.docx` and reopen: freeform shapes with
   several closed paths are the part of the pipeline I could not exercise.
4. Regenerate screenshots (`docs/screenshots.md`).
5. Build the MSI/DMG/Flatpak on their platforms.

## 9. Round 2: fixes after the first push (CI and local build)

Found by pushing the first zip to GitHub and building it on Windows:

- **`Cargo.lock` was incomplete.** `egui`, `emath` and `epaint` had no `source`/`checksum` lines (a
  leftover of temporary dependency patches used while testing in a sandbox). `cargo` re-locked them on
  your machine, and CI, which runs with `--locked`, refused with "cannot update the lock file".
  Fixed by regenerating the lockfile with plain `cargo` (six lines added); `cargo metadata --locked`
  now passes.
- **Clippy (`-D warnings`) would have failed next.** Fixed the six warnings it reports: one of mine
  (a one-element loop in the Backstage footer) and five in upstream code that older compilers flag:
  `math_linear.rs`, `docbin/src/fmt.rs`, `proof/src/ru/mod.rs`, `ui-egui/src/objects.rs`. All fixes
  are behaviour-preserving (De Morgan simplifications, an unused binding).
- **`khata.zip` had been committed** into the repository. `.gitignore` now ignores `*.zip`.
- **Line-ending warnings:** added `.gitattributes` (LF everywhere, binaries untouched).
- **Windows `link.exe not found`:** not a code problem. The Visual Studio C++ Build Tools are
  required (see the setup guide in the chat).

## 8. Known limits

- About 70 shapes, not Word's 170+; thumbnails are outlines; callout tails and braces are simplified.
- No Bengali translation of the interface yet (the font is ready, the catalog is not).
- Some translations fall back to English for strings that were changed.

## 10. Automatic releases (round 3)

- `.github/workflows/release.yml`: one click (Actions › Release › Run workflow › type a version) bumps
  the version, tags it, builds the Windows MSI + portable zip, Linux AppImage/deb/rpm/tar.gz, macOS dmg
  and the web build, and publishes a GitHub Release with checksums and a change list generated from
  commit messages. Pushing a `vX.Y.Z` tag does the same. An empty version is a dry run.
- Windows and Linux are required; macOS and web are best effort; MSI/AppImage failures fall back to
  the portable zip / deb + tar.gz. The web build is also deployed to GitHub Pages as a live demo.
- `.github/dependabot.yml` keeps the workflow actions current (one grouped PR a month).
- `docs/releasing.md` rewritten for this flow; README gained Install, Download and live-demo links;
  `.github/social-preview.png` is a ready-made repository social image.
- Verified: the workflow YAML parses and `cargo xtask assets` passes (53 assets). **Not verified:**
  the workflow has never run on GitHub. Do a dry run first (see docs/releasing.md).
