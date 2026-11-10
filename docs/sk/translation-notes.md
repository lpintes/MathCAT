# Slovak (sk) speech translation: working notes

Living document for the Slovak localization of the MathCAT speech rules.
It records decisions, findings and the work plan so that work can continue
across sessions. Braille is out of scope for now.

Branch: `sk` (based on `main`).

## How a language is wired in (verified from the pl and ja additions)

- Rules live in `Rules/Languages/sk/` with the same file set as `en`:
  `ClearSpeak_Rules.yaml`, `SimpleSpeak_Rules.yaml`, `definitions.yaml`,
  `navigate.yaml`, `overview.yaml`, `unicode.yaml`, `unicode-full.yaml`,
  `SharedRules/{default,general,calculus,geometry,linear-algebra}.yaml`.
- `Cargo.toml`: add `lang-sk = []` and put `"lang-sk"` into `lang-all`
  (alphabetical). `build.rs` fails if a rules directory has no feature.
- `tests/languages.rs`: register `mod sk;` (or a nested `mod sk { ... }` block
  when the tests are split into several files, as ja does).
- Tests: `tests/Languages/sk.rs` plus `tests/Languages/sk/` copied from `en`,
  with `"en"` replaced by `"sk"`. Run with `cargo test Languages::sk`.
- No change to `src/` was needed for pl or ja. Polish added
  `Rules/Languages/pl/intent.yaml` (a per-language intent override), so that
  mechanism exists if Slovak grammar ever needs it.
- Audit against English: `uv run --project PythonScripts audit-translations sk`
  (`--source pl` compares against Polish instead).
- Listening locally: copy `Rules/Languages/sk` into
  `NVDA\include\nvda-mathcat\assets\Rules\Languages` (installed NVDA needs
  admin rights). Test expressions: https://daisy.github.io/MathCAT/test-exprs.html
- Translator guide: `docs/helpers.md`.

## Conventions

- `t:` / `ot:` / `ct:` / `tc:` / `spell:` / `pronounce:` lowercase means
  untranslated or unverified; uppercase (`T:` ...) means verified by a human.
  Machine drafts stay lowercase until the user has approved them.
- Never demote `T:` back to `t:` unless asked.
- Do not change test goldens unless the user says so.
- Rust: explicit `return` in functions; do not run `cargo fmt`.
- No git commands unless the user asks.

## Starting point

Decided (2026-11-10): copy from `en`, and port grammar mechanisms from `pl`
one rule at a time when a Slovak test needs them.

- `en` is the reference structure, so the audit stays clean.
- `pl` already handles the same plural pattern as Slovak (1 / 2-4 / 5+) and
  case after prepositions (`tests/Languages/pl/preposition_case.rs`), but
  copying Polish wholesale would leave near-Slovak Polish strings behind.

Other decisions (2026-11-10):

- Both speech styles, ClearSpeak and SimpleSpeak, are translated together.
- Review happens by email in the user's group, not by upstream PRs per phase.
  Each phase therefore ends with a review package (see below). An upstream
  PR comes only at the end.
- No authoritative Slovak source for reading whole expressions aloud exists
  (web research 2026-11-10, details and URLs in `docs/sk/terminology-sources.md`).
  For single symbol names the 2021 Slovak Braille rules (SKN Levoča,
  ISBN 978-80-18-10144-5) are authoritative; NVDA's Slovak `symbols.dic` adds
  more but has errors. Structure readings (powers, roots, fractions,
  dy/dx, definite integral) are backed by school/university pages and a 2009
  thesis on the Slovak Lambda editor; limits, sums, absolute value, f(x), f′
  and subscripts are unsourced. Terminology is proposed from these, reviewed
  by the group, and recorded in "Terminology decisions".

## Review package for the email group

For each phase, produce one HTML file (e.g. `docs/sk/review/phase-N.html`)
that lists, per example: the rendered MathML (so a sighted reviewer sees the
formula and an NVDA user can read it with MathCAT), the Slovak speech in
ClearSpeak and SimpleSpeak, and the English speech for comparison. Reviewers
reply with the item number and their proposal.

`PythonScripts/rust-test-to-html.py` already extracts MathML from Rust tests
but writes it as escaped text without the expected speech; it can serve as a
starting point for a small generator.

## Slovak grammar issues to expect

- Plural forms after numbers: 1 meter, 2-4 metre, 5+ metrov (units, "krát").
- Numeral gender: jeden/jedna/jedno, dva/dve.
- Case after prepositions: "od a do b", "z x" (odmocnina z x),
  "pre x idúce k nule", "na druhú".
- Fractions with ordinals: jedna polovica, dve tretiny, tri pätiny, ...
- Powers: "x na druhú", "x na tretiu", "x na n-tú".
- Diacritics must reach the speech synthesizer unchanged.

## Terminology decisions

Record each settled term here: English phrase, Slovak phrase, reason/source.

(none yet)

## Work plan (each phase is one reviewable unit)

1. Skeleton: copy rules and tests, register `lang-sk`, build and run tests
   (failures expected). No translation yet. Also create the review-package
   generator, so that every later phase can be sent out the same way.
2. Vocabulary: `definitions.yaml` (ordinals, fraction denominators, function
   names) and `unicode.yaml` (Latin and Greek letters, digits, common
   operators and relations).
3. Core structures: `SharedRules/general.yaml` and `default.yaml` (fractions,
   roots, powers, scripts, fences, tables) with the corresponding tests.
4. ClearSpeak: `ClearSpeak_Rules.yaml` and its tests, including preferences.
5. SimpleSpeak: `SimpleSpeak_Rules.yaml` and its tests.
6. Topic rules: `calculus.yaml`, `geometry.yaml`, `linear-algebra.yaml`.
7. Grammar agreement: units and plural forms, case after prepositions;
   new Slovak-specific tests.
8. Navigation and overview: `navigate.yaml`, `overview.yaml`.
9. `unicode-full.yaml`: recognisable symbols only.
10. Review: mathematician review, user testing with NVDA, final audit,
    upstream pull request.

## Status

- Phase 1: skeleton done (rules and tests copied from `en`, `lang-sk`
  registered). 593 of 600 copied tests pass; the 7 failures are the
  decimal-comma difference described below. Review-package generator: to do.

## Findings and dead ends

- Run tests with `MATHCAT_IGNORE_USER_PREFS=1`, e.g.
  `MATHCAT_IGNORE_USER_PREFS=1 cargo test --test languages Languages::sk`.
  Otherwise MathCAT reads the user's own `%APPDATA%\MathCAT\prefs.yaml`
  (located via `dirs::config_dir()`, so redirecting `APPDATA` does not help).
  On this machine that file is an old one from 2023 without the `Other` key,
  and every test, English too, fails with "Didn't find 'Other' key".
- With `DecimalSeparator: Auto`, Slovak uses a decimal comma (`sk` is not in
  `USE_DECIMAL_SEPARATOR` in `src/prefs.rs`), so `.` acts as a thousands
  separator: `<mn>5.0</mn>` is spoken as 50. This is correct for Slovak. The
  copied English tests that use `.` in numbers (`decimal_power`,
  `msub_not_simple`, `msubsup_not_simple`, `si_other_numbers`,
  `test_mtext_inference`) must get Slovak input such as `5,0` when they are
  translated. `ignore_period_and_space` also fails under `sk`: the final
  "dot" is not spoken. Probably the same separator logic; not yet verified.
- The copied test files lose CRLF when edited with Git Bash `sed -i`; the repo
  has `*.rs`/`*.yaml text eol=crlf`, so restore with `unix2dos` after such edits.
