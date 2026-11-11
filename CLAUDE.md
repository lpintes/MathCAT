@AGENTS.md

## Slovak (sk) localization (branch `sk`)

- Goal: Slovak speech rules in `Rules/Languages/sk/` (ClearSpeak and SimpleSpeak). Braille is out of scope.
- Read `docs/sk/translation-notes.md` first: plan, decisions, findings and terminology decisions.
  Keep it up to date; it is the memory of this work across sessions.
- Terminology sources: `docs/sk/terminology-sources.md`. Do not invent readings; unsourced
  proposals stay lowercase `t:` until the user's review group approves them.
- Review happens by email in the user's group, one HTML review package per phase; the
  upstream pull request comes only at the end.
- Remotes: `origin` is the user's fork `lpintes/MathCAT`, `upstream` is `daisy/MathCAT`.
  Every approved commit on `sk` is followed immediately by `git push` to `origin`, so the
  fork always has all changes; approving a commit also approves that push. Never push to
  `upstream`. Merging from upstream: "Fork and upstream sync" in `docs/sk/translation-notes.md`.
- Run tests with user prefs ignored (the local `%APPDATA%\MathCAT\prefs.yaml` is outdated):
  `MATHCAT_IGNORE_USER_PREFS=1 cargo test --test languages Languages::sk`
- `*.rs` and `*.yaml` are CRLF in the working tree; Git Bash `sed -i` drops CR, restore with `unix2dos`.
