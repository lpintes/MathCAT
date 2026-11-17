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
- Test goldens: in `tests/Languages/sk/` the user has given standing permission
  (2026-11-10) to replace the expected strings with readings approved by the
  review group. Approved means listed as approved in a review YAML file.
  Goldens of other languages stay untouched.
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

Decided (2026-11-10). Main principle: once the group approves a reading, it is
never reviewed again unless the output changes.

- Items are hand-written YAML, one file per topic, e.g.
  `docs/sk/review/fractions.yaml`. Items may be copied from the tests as a
  starting point. Each item has a stable ID (`frac-003`), the MathML, an optional
  note for reviewers and, after review, the approved Slovak text per style.
- A generator (new bin `review-package`, own `[[bin]]` in `Cargo.toml`; leave
  `src/bin/mathml2text.rs` unchanged) loads the rules once. It speaks each item in
  sk and en, ClearSpeak and SimpleSpeak, and writes one HTML file. Per item the HTML
  shows the rendered MathML (a sighted reviewer sees the formula, an NVDA user
  reads it with MathCAT), sk speech in both styles and en speech for comparison.
- The generator compares the current output with the approved text. Only new
  and changed items go into the package; changed ones show old and new text.
  Approved, unchanged items are left out, or listed collapsed at the end.
- The package header has a "Known issues, do not report" section for defects
  already known but not fixed yet.
- Packages are small (about 30-60 items). No reply deadline: the review work
  is voluntary and nobody is chased (user, 2026-11-11), so build packages
  without `--deadline`. Reviewers reply only about items they want to
  change: item ID, the word concerned and a proposal. Items nobody mentions
  count as approved (user, 2026-11-11). When reviewers disagree, the
  user decides; the decision and its reason go to "Terminology decisions".
- After review: fix, send a short follow-up package if outputs changed, write
  the approved texts into the YAML and into the sk test goldens, commit.
  From then on `cargo test` guards approved readings against regressions.

`PythonScripts/rust-test-to-html.py` extracts MathML from Rust tests (as
escaped text, without speech); useful for seeding item YAML from tests.

### Generator usage

- Item files: `docs/sk/review/<topic>.yaml`; the format is described in the
  header comment of `src/bin/review_package.rs` (keys `topic`, `known_issues`,
  `items` with `id`, `mathml`, optional `note`, `styles`, `prefs`, `approved`).
  `docs/sk/review/fractions.yaml` is the first, still unapproved, trial file.
- Build a package:
  `MATHCAT_IGNORE_USER_PREFS=1 cargo run --bin review-package -- docs/sk/review/fractions.yaml`.
  Output goes to `target/review/<topic file stem>.html` unless `-o` is given.
  The environment variable is needed for the same reason as in the tests; the
  bin does not set it itself because `std::env::set_var` is `unsafe` in Rust 2024.
- Base preferences are the same as in `tests/common/mod.rs` (Verbosity Medium,
  MathRate 100, ...), so the package shows what the goldens check. Per-item
  `prefs` are restored after each item.
- Comparison with `approved` collapses runs of spaces, like the tests do.
- Invalid MathML in an item stops the generator with the item ID, because the
  MathML is copied into the HTML unchanged.
- Approved texts are written into the YAML by hand (no write-back yet).
- `--all` shows every item in full, approved ones too, instead of the collapsed
  list at the end; used for the final check package of a topic
  (`-o target/review/<topic>-final.html`).
- Idea on hold, maybe never built: interactive review page with inline
  comments and per-item status, see `docs/sk/web-app.md`.

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

Glossary round 1 (`docs/sk/review/vocabulary.yaml`, replies 2026-11-16, decided
2026-11-17). Items nobody commented on are approved as drafted; their words are
now `T:` and all 58 items have `approved:` texts.

- nu (ν): "ný", not "ní". eSpeak reads "ní" as "ňí"; the spelling is phonetic
  on purpose.
- Relations take the neuter gender (group consensus): "je menšie ako", "je väčšie
  ako", "je menšie alebo sa rovná", "je väčšie alebo sa rovná", "je totožné s", "je
  úmerné s", "je rovnobežné s", "nie je rovnobežné s". Round 2 (2026-11-17) changed
  ≤ ≥ from "menšie alebo rovné" to "menšie alebo sa rovná" and added "s" after
  "úmerné".
- "je" before relations: ClearSpeak says it; SimpleSpeak and Verbosity Terse drop it
  (round 2): "a menšie ako b", "a menšie alebo sa rovná b", "a totožné s b",
  "a úmerné s b", "p rovnobežné s q", and ∦ becomes "p nerovnobežné s q". Applied to
  < > ≤ ≥ ≡ ∝ ∥ ∦; other relations (∈, ⊂, ...) still follow the old rule and are
  decided in the relations (sets and logic) package.
- Fences: ClearSpeak and the other styles say "ľavá/pravá zátvorka", "ľavá/pravá
  hranatá zátvorka", "ľavá/pravá zložená zátvorka". SimpleSpeak and Verbosity Terse
  use the NVDA `symbols.dic` words: "zátvorka" / "zatvoriť", "hranatá zátvorka" /
  "hranatá zatvoriť", "zložená zátvorka" / "zložená zatvoriť".
- Arrows: "šípka doľava/doprava/nahor/nadol" (direction adverbs, as in CLDR);
  "dvojitá šípka doprava". The filled arrows U+2B05-2B07 in `unicode-full.yaml` are
  "plná šípka doľava/nahor/nadol" (group proposal, still `t:`: not in a package).
- Asterisk: "hviezda" (NVDA, RHVoice, eSpeak).
- Percent sign on its own: "percent" (as after a number, like "stupňov"); the
  plural forms come with the units topic.
- Fraktur: the flag word "fraktúra" before the letter, like "veľké" ("fraktúra
  veľké r"). Double-struck: "dvojito prečiarknuté" (reviewer term); for the italic
  ⅆⅇⅈⅉ "dvojito prečiarknuté kurzíva d" (round 2 changed "kurzívne" to "kurzíva"). The flag was also changed for ℂℕℚℝℤ
  (as letters, not as set names) for consistency.
- Function application: no connector after named functions (trig, hyperbolic,
  log, log with a base, exp, Cov, Re, Im) in any style and with any argument:
  "sínus x", "sínus, ľavá zátvorka, x plus y, pravá zátvorka", "logaritmus so
  základom b; x". All reviewers asked for it; "z" was mentioned but is risky.
  At first only SimpleSpeak; the user extended it to ClearSpeak on 2026-11-17
  (English ClearSpeak keeps "of" there, Slovak has no reason to).
- "od" is not wanted anywhere (user decision, 2026-11-17). Still left: the
  general function "f od x" (U+2061 rule in `unicode.yaml`) and any "od" that
  appears in later topics. Remove it in the functions package; check there
  that "f x" is not confused with implied multiplication (f krát x), e.g. by
  a pause. Do not propose "od" again in new rules. ("z" in "absolútna hodnota
  z x", `FunctionApplicationWord`, is a different word and not covered.)
- Fraction numerators (2026-11-17, user): the nouns are feminine, so 1 and 2 are
  spoken as words "jedna", "dve" (some synthesizers read "1" as "jeden", giving
  "jeden polovica"); other numerators stay digits. Numerators 1-4 take the
  nominative (dve tretiny), the others the genitive plural (päť tretín, nula
  polovíc) from `FractionGenitivePlural` in `definitions.yaml` (denominators
  2-19). Rules: `common-fraction` (both styles), `common-fraction-mixed-number`
  (SimpleSpeak).
- Fraction nouns for denominators of 20 and more are broken; fix them first in the
  fractions package (user, 2026-11-17: "dvadsaťpäť stotín" is common, e.g.
  25/100 when simplifying fractions). They are reached with
  ClearSpeak_Fractions Ordinal and with SimpleSpeak mixed numbers (any
  denominator). `ToOrdinal` composes them the English way, word by word, and
  with the wrong lists. Output on 2026-11-17: 1/20 "jedna dvadsiata", 1/25
  "jedna dvadsať piata", 1/100 "jedna stá", 1/1000 "jedna jeden tisíca",
  3/21 "3 dvadsať celky", 3/22 "3 dvadsať polovice", 25/100 "25 stotiny".
  Planned fix, in the rules only (no Rust change): stop using `ToOrdinal` for
  fraction denominators. Three dictionaries in `definitions.yaml` keyed by the
  denominator (singular, nominative plural, genitive plural): 2-19, the tens
  20-90, 100, 1000, 1000000 (dvadsatina/dvadsatiny/dvadsatín,
  stotina/stotiny/stotín, tisícina/tisíciny/tisícin, milióntina/milióntiny/
  milióntin). The fraction rules match only denominators found there; others
  fall through to the "lomeno" reading. Compounds (21-99, e.g. dvadsaťpätina,
  and 22 with "dvojina"?) only from a source, see `terminology-sources.md`.
  Genitive length (stotín but tisícin, pätnástin) follows the rhythmic law, so
  write the forms out, do not derive them.
- Script letters "písané", "koncová sigma", "polovica", number set names and the
  remaining symbol names: approved as drafted.
- ± (voc-012): "plus alebo mínus" (reviewer proposal), because "a plus mínus b"
  can also mean a + (−b). Accepted although the user generally prefers short
  readings.

## Work plan (each step is one review package)

Revised 2026-11-10: packages are organised by topic, not by rules file. Each
topic covers ClearSpeak and SimpleSpeak together and touches whichever files it
needs (`definitions.yaml`, `unicode.yaml`, `SharedRules/*`, `*_Rules.yaml`).
Grammar (case after prepositions, plural after numbers, numeral gender) is
handled within each topic, not in a separate late phase, so reviewers do not
report the same grammar errors in every package.

1. Skeleton: copy rules and tests, register `lang-sk`, build and run tests
   (failures expected). No translation yet. Then build the `review-package`
   generator and the item YAML format.
2. Vocabulary only, no expressions: Latin and Greek letters, digits, common
   operators and relations (`unicode.yaml`), ordinals, fraction denominators,
   function names (`definitions.yaml`). A glossary list, quick to review; the
   results go to "Terminology decisions" and the words move from `t:` to `T:`.
3. Topic packages, in roughly this order: fractions; powers, roots and
   scripts; fences and absolute value; functions and f(x), f′; sums, products and
   limits; calculus (derivatives, integrals); sets and logic; geometry;
   linear algebra (matrices, determinants, tables); units and numbers
   (plural forms, decimal comma); ClearSpeak preferences.
4. Navigation and overview: `navigate.yaml`, `overview.yaml`.
5. `unicode-full.yaml`: recognisable symbols only.
6. Final: user testing with NVDA, final audit, upstream pull request.

## Fork and upstream sync

- Remotes: `origin` is the fork `lpintes/MathCAT` (branch `sk` is pushed and
  tracks `origin/sk`); `upstream` is `daisy/MathCAT`. The fork's `main` is not
  maintained; merge straight from `upstream/main`.
- Upstream is busy (about 4 commits a day in 2026), mostly other languages.
- Merge, never rebase: `sk` is public and reviewers may refer to its commits.
  If upstream wants a clean history, squash at the final pull request.
- When: before building each review package, at the start of each phase, and
  whenever an upstream fix is needed. That is roughly every 1 to 3 weeks.
- Procedure:
  1. `git fetch upstream`
  2. Relevant changes only:
     `git log --oneline sk..upstream/main -- Rules/Languages/en src tests/languages.rs Cargo.toml`
  3. `git merge upstream/main`. Our files outside `sk` directories that can
     conflict: `Cargo.toml`, `CLAUDE.md`, `src/bin/review_package.rs`,
     `tests/languages.rs`. Check CRLF in `*.yaml` and `*.rs` afterwards.
  4. `MATHCAT_IGNORE_USER_PREFS=1 cargo test --test languages Languages::sk`
     (the pass count must not drop for reasons other than our own changes) and
     `uv run --project PythonScripts audit-translations sk`.
  5. Port the `en` rule changes the audit reports into `sk` right away
     (`git diff <last merge base> upstream/main -- Rules/Languages/en` shows
     them); keep new words as `t:`. Record open translation tasks in "Status"
     below, then `git push`.

## Status

- Phase 1: skeleton done (rules and tests copied from `en`, `lang-sk`
  registered). 593 of 600 copied tests pass; the 7 failures are the
  decimal-comma difference described below.
- Review strategy and revised work plan decided (2026-11-10).
- Review-package generator done (`src/bin/review_package.rs`, bin
  `review-package`), tried on `docs/sk/review/fractions.yaml` and on a scratch
  file covering approved, changed and new items and invalid MathML.
  Phase 1 complete; next is phase 2 (vocabulary).
- Phase 2 drafts (lowercase `t:`, unreviewed): `unicode.yaml` complete;
  `definitions.yaml` IntentMappings, `FunctionApplicationWord` ("z") and the
  number lists. Units, `NavigationParts` and `KnownWords` are still English
  (units topic and phase 4). `SharedRules/general.yaml`: number sets, real and
  imaginary part, trigonometric and hyperbolic names, exp, Cov, log/ln drafted;
  the rest of that file is still English.
- Glossary package `docs/sk/review/vocabulary.yaml` (58 items) built and sent
  to the group (2026-11-11). Round 1 replies applied (2026-11-17), see
  "Terminology decisions": all 58 items approved. The changed outputs are what
  the reviewers asked for, so instead of a follow-up package a final check
  package with every item (`--all`, `target/review/vocabulary-final.html`) goes
  to the group; replies to it are handled like a normal round.
- Question for the group: are the localization XML files of the Slovak Lambda
  editor available (they may hold more readings)? The user is looking for them.
- First upstream merge (2026-11-11, up to a4ae93ed): no conflicts, sk tests
  unchanged (173 pass / 427 fail, English goldens). Task for the units topic:
  upstream 33c3ed4f reworked units and currency (currency over a unit such as
  $5/h, `$` now singular "dollar" with plural via unit rules, new
  `CompoundUnitPlural` / `NumberlessUnitPlural` keys in `Rules/definitions.yaml`).
  The audit reports one rule missing in `sk` from that change; port it and
  choose the Slovak compound-plural policy.
- Second upstream merge (2026-11-17, up to c274aa13, merge 24a534d6): no
  conflicts, sk tests unchanged (173 / 427). All `en` rule changes since the
  skeleton (71612e4c) are now ported to `sk`; the audit shows the same rule
  counts in every file. Ported: currency over a unit in `per-fraction`
  (ClearSpeak, SimpleSpeak), `unit-grouping-paren`, `currency-symbol`, the
  reworked `currency` rule and the `CurrencySymbols` branch of `unit`
  (`general.yaml`), `:literal:` and `SpeakIntentName` in `default-text`
  (`default.yaml`), the `DEBUG` removal in `navigate.yaml`, `μ` (U+03BC) in
  `SIPrefixes` and the plural-policy comment (`definitions.yaml`). New words
  are still English `t:`. Still open for the units topic: the Slovak values of
  `CompoundUnitPlural`, `NumberlessUnitPlural` and `VariableBeforeUnitPlural`
  (Slovak needs case forms after numerals, which these keys alone cannot express).

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
- `ToOrdinal` (`src/xpath_functions.rs`) uses the same `NumbersOrdinal*` lists
  for powers/roots and for fraction denominators. The `Plural` lists are only
  used for fractions, so in sk they hold plural nouns (tretiny); the singular
  lists hold the feminine nominative adjective (tretia). The `Fractional` lists
  are filled for 0-19 so small denominators get nouns (tretina). Only a binary
  singular/plural choice exists; Slovak 2-4 vs 5+ (tri tretiny, päť tretín) and
  the accusative after "na" (na tretiu) need rule work in the fractions and
  powers topics.
- IntentMappings strings in `definitions.yaml` have no `t:`/`T:` marker, so their
  review status lives only in the review YAML files.
- `FunctionApplicationWord` (named functions: "absolútna hodnota z x") and the
  U+2061 word in `unicode.yaml` (f od x) are separate; `log` with a base uses
  U+2061, giving "logaritmus so základom b, od x". A question for the glossary.
- `TerseFunctionNames` lists intent names, not spoken text; leave it unchanged.
- Vocabulary is not only in `unicode.yaml` and `definitions.yaml`: `SharedRules/general.yaml`
  speaks number sets (ℝ alone is "reálne čísla", not the letter from `unicode.yaml`) and
  the trig/hyperbolic/log names for `mi` elements.
- Review items: binary operators grouped with `mspace` in one item get confusing pauses
  ("a sa rovná, b a sa nerovná b"); give each operator its own item. Letters group well.
  A function name without U+2061 and an argument gets "od" appended.
- Still English after phase 2 (for the topic packages): implied "times" and "the empty
  set", "such that" (`ClearSpeak_Rules.yaml`, `SimpleSpeak_Rules.yaml`); ClearSpeak "log"
  and "natural log"; "absolute value of"; "the cube root of"; "of" in "pomer of 3 ku 4"
  (`SharedRules/default.yaml`); `unicode-full.yaml` (∼ "varies with", ⌊⌈ floor/ceiling);
  arcsin/arctg are spoken as letters; gcd/lcm are not recognised as "NSD"/"NSN"; Slovak
  "cotg" is not matched by the cot rule in `general.yaml`.
- Without "od", both styles put a pause after multi-word function names
  ("hyperbolický sínus, x", "prirodzený logaritmus, x") but not after one-word
  names ("sínus x"). The pause is not in the rules; probably the automatic pause
  length. The approved texts in voc-053/054 contain it; look at it in the
  functions package.
- The "od" skip is in the U+2061 rule in `unicode.yaml`. Cov, Re, Im
  and log with a base need their own tests there because canonicalization turns them
  into `real-part` / `imaginary-part` / `msub` nodes and Cov is in no function list.
- A YAML comment inside a multi-line double-quoted XPath string is part of the
  string; a `"` in it ends the string and the rules file fails to parse.
