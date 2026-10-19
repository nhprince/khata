# Licensing and legal housekeeping

*This is a practical checklist, not legal advice. For trademark or patent questions, or before
commercial use, ask a lawyer in your country.*

## What license Khata uses

Khata is **dual-licensed under MIT OR Apache-2.0**: anyone may use, modify and redistribute it under
either license, at their choice (`LICENSE-MIT`, `LICENSE-APACHE`). That is the same choice the
upstream WordCraft project made, and all 538 third-party Rust crates in the build use permissive
licenses (MIT, Apache-2.0, BSD, ISC, Zlib, Unicode, plus one MPL-2.0 crate), which are compatible with
it. `python3 packaging/third-party-licenses.py --check` verifies this and runs in CI, so a new
copyleft or unknown license fails the build instead of slipping in.

Both licenses already contain the warranty disclaimer ("as is", no liability) that protects you as the
author. The Apache-2.0 option also gives users a patent license, which MIT does not.

## What you must do (and where it is done)

| Obligation | Where |
|---|---|
| Keep the upstream copyright and license notices | `LICENSE-MIT`, `LICENSE-APACHE` unchanged; `contributors/`; README "Credits" |
| Ship the `NOTICE` file with copies you distribute (Apache-2.0 §4(d)) | `NOTICE`, copied into every package |
| State that you changed the files (Apache-2.0 §4(b)) | `NOTICE`, `REBRAND_CHANGES.md`, git history |
| Ship the licenses of third-party code and fonts | `THIRD_PARTY_LICENSES.txt`, generated on every release, in the packages and attached to the release |
| Keep font licenses with the fonts (SIL OFL 1.1) | `assets/fonts/OFL-*.txt`; fonts are bundled unmodified and not sold on their own |
| Record every new asset's author, source and license | `ATTRIBUTION.md`, enforced by `cargo xtask assets` |

## Names, logos and trademarks

- The **ArtCraft** and **WordCraft** names and the ArtCraft logos are not licensed to you. They are
  not used in Khata; the only mentions are the factual credits (README, About, NOTICE).
- **Microsoft** and **Word** are Microsoft trademarks. Khata does not use them as a product name, and
  every page that mentions them says Khata is not affiliated. Keep it that way ("opens .docx files",
  not "Microsoft Word compatible" in a logo or name).
- **Khata** (খাতা) is an ordinary word meaning notebook or ledger, and unrelated finance apps also use
  it. A license does not give you a trademark; rights come from use, or from registering the name.
  For a portfolio project the risk is low. Before you promote it widely or sell anything, search the
  trademark registers of your country and the countries you target, and consider a registration.
  Using the full form "Khata, a notebook-style word processor" helps distinguish it.

## Your own contributions

- Your changes, the logo and the artwork are yours (© NH Prince Pradhan, stated in `README.md` and
  `NOTICE`) and are offered under the same dual license.
- Code and artwork in this project were produced with AI assistance. In some countries (the United
  States, for example) purely AI-generated material may not be protected by copyright, while your
  creative selection, direction, edits and original parts are. Keep your commit history and notes as
  evidence of your own contribution.
- Pull requests from others are accepted under the same license (`CONTRIBUTING.md`).

## Privacy

Khata has no telemetry and no update checker, and makes no internet connections of its own (only
local ones to the optional Zotero connector and the optional automation port). If you ever add anything that sends data, update that statement and add
a privacy policy.

## Reporting security problems

See `SECURITY.md`.
