# Interactive review: idea on hold

Status: idea only, discussed 2026-11-11. Nothing is decided and it may never be
built. The email review with static HTML packages (see "Review package for the
email group" in `translation-notes.md`) stays the process. Revisit only after
the group's first feedback, and only if reviewers find the email workflow
tiring.

## Goal

Let reviewers work incrementally, whenever they have time: comment on an item
in place, mark items as seen, and continue later where they stopped.

## Interaction design (screen reader first)

- Comments are added inline, not in a new window or popup, which disorients
  screen reader users.
- Each item has a button whose name includes the item ID, e.g.
  "Add comment to frac-012", so the NVDA elements list (NVDA+F7) shows distinct
  names. The button uses `aria-expanded`.
- Activating it reveals a text area right below and moves focus into it, with
  "Save" and "Cancel" buttons. After saving, focus returns to the button and an
  `aria-live` region announces "Saved".
- Each item has a status chosen by radio buttons: not reviewed, OK, has
  comment.
- Each item is a heading, so H moves between items. The status is part of the
  heading text, e.g. "frac-012, fraction, not reviewed", so it is heard while
  skimming.
- Top of the page: progress ("23 of 58 reviewed") and a "Show only
  unreviewed" filter. With the filter on, H moves only through unreviewed
  items, so no separate "next unreviewed" link is needed.
- Status is never shown by colour alone. Rendered MathML stays as it is now.

## Rules that must not change

- Items nobody comments on still count as approved. "OK" only tracks the
  reviewer's own progress; it must not turn voluntary work into an obligation
  to tick every item.
- An item whose output changed goes back to "not reviewed" automatically
  (matches "never reviewed again unless the output changes"). Key the stored
  status by item ID plus a hash of the spoken outputs.

## Options, simplest first

1. Static HTML, no server. The `review-package` generator adds the forms;
   state is stored in the browser (`localStorage`). A "Prepare reply" button
   collects the comments into text and copies it to the clipboard for the
   email. No accounts, passwords or hosting; the email process stays. Downside:
   unfinished work lives in one browser on one computer.
2. Small own web app with a server. Instead of name and password, one personal
   link per reviewer with a secret token in the URL (no forgotten passwords, no
   login form). Data in SQLite. The app reads the same
   `docs/sk/review/*.yaml` files and can write results back (fill `approved:`,
   collect comments). Cost: hosting, maintenance, responsibility for personal
   data.
3. Existing services (GitHub Issues/Discussions, Google Forms): not
   recommended. GitHub needs accounts and is heavy for ordinary reviewers;
   forms do not map well to individual items.

## Recommendation

Start with option 1: a small extension of the generator, nothing to operate,
and it shows whether reviewers actually use statuses. Move to option 2 only if
the group asks for working from several devices; item rendering, statuses and
export carry over.
