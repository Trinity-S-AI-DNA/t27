# T27 OS secrets

Law: `specs/t27os/secrets.t27` (issue #13954). Provider rules stay in `specs/t27os/keys.t27`.

- **Store:** `<home>/.t27os/keys`, one file per secret. `~/.dsh/` and `.env` files are not the store.
- **Reference form:** `t27os:keys/<name>`; a consumer keeps the reference, never the value or a path.
  The runner token is `t27os:keys/gitlab-runner`.
- **Rules:** name checks, resolver, redaction and the store policy are in the spec; read them there.

## Migration order

1. Law first: the spec above (this slice). No file on disk moves and no consumer changes.
2. One issue per consumer, each verified to load from the store before the next starts:
   DSH, OpenCode, FCC, t27-lagger, t27-ladder.
3. Duplicates (`GLM_API_KEY`, `GLM_API_KEYS`, `RENDER_API_KEY`) are removed only after their
   consumer is verified to load from the store.

No secret value goes into an issue, a log, a PR or this page.
