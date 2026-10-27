---
id: secret-hygiene
name: Secret hygiene
description: Find, list and remove plaintext secrets without ever reading or printing a value -- a names-only inventory with jq and grep -o, a rotation list that holds names and places only, in-place removal from settings.json, and the traps (old backup copies, editor swap files, tool output that already showed a value). Use when the owner says "find all the keys", "rotation list", "remove the secrets", "revoked", or when a secret appears in tool output.
argument-hint: "[inventory | list | remove | after-exposure]"
---

# Secret hygiene (owner requests, 2026-10-25)

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

The rule behind it is the owner's "never commit a secret" (gitleaks in lefthook and CI, canonical rules in
`.gitleaks.toml`); this page is the order of moves when a secret is already on disk. The shell here is
`jq` and `grep`, which are tools, not code written in another language.

## The moves

1. **Inventory by name, never by value.**
   - JSON config: `jq -r '[paths(scalars)|map(tostring)|join(".")]|.[]' <file>` prints key paths only.
     Filter the paths with `grep -E 'TOKEN|KEY|SECRET|PASSWORD|URL'`; never print `jq '.env'`.
   - `.env`-style files: `grep -o '^[A-Za-z_][A-Za-z0-9_]*=' <file>` prints the text up to and including the
     `=`, no more. Use `cut -d= -f1` only after `grep -o` so the value never enters the pipe.
   - Presence of credential files: `ls -l` on explicit paths, with an existence test, not a glob (an unmatched
     glob aborts zsh).
   - `gh auth status` with its token line filtered out; the Keychain is not opened (account names come from
     `specs/t27os/keys.t27`). Reading a `.env` or the Keychain to test whether a key is alive is a probe the
     owner may refuse; ask for the KEYS-tab check instead.
2. **Write the rotation list** in the session worktree root, untracked, LOCAL ONLY (it names home paths and is
   an inventory an attacker wants; the home-path gate would refuse it anyway). Columns: name, what it opens,
   where to rotate, what to do after. Mark which names were seen in tool output: those are exposed.
3. **The owner rotates.** Revoking is a click in the provider's console, with a password or sign-in that is
   the owner's alone. Do not paste a new value anywhere: not chat, not an issue, not a PR.
4. **Remove only after the owner says revoked.** For a JSON file, in place and without a copy:
   - `jq 'del(.env.NAME, .mcpServers["srv"].env.NAME2)' <file> > <file>.new`, then check that it parses and that
     the scalar count fell by exactly the number deleted (`jq '[paths(scalars)]|length'`);
   - check by name that none of the deleted names remain;
   - `cat <file>.new > <file>` (keeps the inode and the mode, which was 600), then `rm <file>.new`.
   - Do not `cp` a backup first: it is one more plaintext copy. The owner's revocation is the safety net.
5. **Look for the copies.** `ls <dir>/<file>*` for `.bak`, dated copies and editor swap files (`.swp`), and
   `grep -c` the secret names in each (a count, not a match). Delete them only on the owner's word: a
   permanent delete is theirs to approve, and a swap file may belong to an open editor.
6. **Verify** with the names-only check again, `ls -l` for mode 600, and the tools that used the secret
   (`gh auth status`, a fresh pane). Then update the memory note and the list.

## After exposure

- A value that appeared in tool output is burned for this session's transcript: tell the owner once, by name,
  and put it first in the rotation order. Never repeat it, even to quote it.
- A secret that was ever pushed is burned: deleting it from the tree does not unpublish it.
- Transcripts under `~/.claude/projects` can hold it too. A grep for common token shapes (`ghp_`,
  `github_pat_`, `gho_`, `sk-ant-`) counts files without printing lines; a zero is not proof of a clean
  transcript, only that those shapes are absent.

## Traps

- `jq '.env'`, `cat`, `head` and `grep` without `-o` or `-c` on a secret file all print values. The first
  command on an unknown file is `jq` for paths or `grep -c`.
- Four older `settings.json.bak*` copies held the same secrets as the live file on 2026-10-25; the live file was
  clean and the copies were not until the owner said delete.
- A config that a secret was removed from may still be needed by the tool: the keyring (`gh`), a credentials
  file (`kaggle.json`) or per-pane tokens usually replace the env line. Check that before deleting.
- Unquoted globs in `ls` and `find` abort zsh with "no matches found"; test each path with `[ -f path ]`.
