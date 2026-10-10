# ghpm — backlog of this repository

This folder is generated and synchronised by **GitHubProjectManager (GHPM)** with the
GitHub project of this repository. Each file of `specs/` is one item of the board
(issue, draft or pull request). Edit the files, then run **Sync specs** in GHPM:
changes are only sent to GitHub at that moment, after a preview.

## Files

- `project.json` — project identity and allowed values (`fields.status`, `fields.size`). Do not edit.
- `index.md` — summary table of the specs. Do not edit.
- `specs/*.md` — one spec per file: a front matter between two `---` lines, then the Markdown body.

## Codes and file names

Every item has a code, at the start of its GitHub title (`US0042 — Login by email`) and of its file name
(`specs/US0042-login-by-email.md`):

- `US0042`: issue #42;
- `SUB004201`: sub-issue n°01 of US0042 (`SUB00420101`: sub-issue n°01 of SUB004201);
- `PR0013`: pull request #13 and `DRAFT-ab12`: draft (file names only, not in the titles).

A code is given once, when the issue is created, and never changes (not even when the issue changes
parent). Keep it at the start of the title when you edit it. GHPM never renames an existing file.

## Editing a spec

```markdown
---
# Managed by GHPM: do not edit these keys
id: PVTI_xxx
rev: 2026-10-08T12:03:11Z
url: https://github.com/owner/repo/issues/42
type: Issue
repository: owner/repo
labels: [auth]
# Editable
title: US0042 — Login by email
status: In progress
size: S
estimate: 3
startDate: 2026-10-01
endDate: 2026-10-10
parent: "#12"
order: 40
---

As a user, I want to sign in with my email…
```

- Editable keys: `title`, `status`, `size`, `estimate`, `startDate`, `endDate`, `parent`, `order` and the body.
- Never change `id`, `rev`, `url`, `labels`, `assignee`, `readOnly`.
- `status` and `size`: one of the names listed in `project.json` (case is ignored).
- Dates: `YYYY-MM-DD`. `estimate`: a number (`3`, `2.5`).
- One `key: value` per line; quote values containing `: ` or ` #` (`title: "Fix: crash"`).
  No multi-line values: long text goes in the body.
- Clearing a value (empty `status:`) is not supported: GitHub's value is kept (except `parent`).
- `parent`: the parent issue (GitHub sub-issues), written `"#12"` (quoted: `#` starts a YAML comment)
  or `"owner/repo#12"` for another repository. You may also give the item `id` of the parent or the
  file name of its spec (`specs/login.md`), including a spec created in the same sync: GHPM rewrites it
  as `"#12"`. Empty `parent:` detaches the issue. Only issues have a parent (not drafts or pull requests),
  and the parent must be an issue of the project.
- Files with `readOnly: true` (pull requests) are rewritten from GitHub.

## Order

`order` is the position of the item on the GitHub board (smaller = higher); `index.md` lists the specs
in that order. Values are spaced by 10: to move a spec, change only its own `order`, e.g. `15` to put it
between the specs at `10` and `20`. At the sync, GHPM moves the item on the board and may rewrite the
`order` of some other specs (new values in between, or a full renumbering when there is no room left).
Two specs with the same value keep their order on GitHub. Moves made on the board reach the files; a spec
moved on both sides is a conflict. Pull requests follow GitHub. File names never change.

## Creating a spec

Create a file in `specs/` (any name, e.g. `specs/login-by-email.md`) **without `id`**:

```markdown
---
type: Issue
title: Login by email
status: Backlog
size: M
---

Description…
```

- `type`: `Issue` (default) or `DraftIssue`; `repository`: `owner/repo` (default: `defaultRepository`).
- Defaults: `status: Backlog`, `size: M`. Give an `order` to place the new item; without it, it goes
  where GitHub adds it, and the next sync writes its actual `order`.
- To split a feature, create the parent spec and its children in one go: give each child
  `parent: specs/<parent file>.md`; the parent is created first.
- Write the title **without code**: at the sync, GHPM creates the issue, adds its code to the title
  (`US0043 — …`, or `SUB004201 — …` when `parent` is set), adds `id`, `rev` and `url` to the file and
  renames it to its standard name (`specs/US0043-login-by-email.md`).

## Not supported

- Deleting an item: a deleted file is written again at the next sync.
- Moving a spec to another project.
