# TASKS — сессия f1d2f458 (Эпик: Лукоморье / t27ai_bot)

Составлено 2026-11-13 из полного лога сессии
(`~/.dsh/sessions/--Users-playra-t27--/session-f1d2f458-eb2b-4d03-a1d3-92868ba7bbe3/session.v4.jsonl.zstd`,
140 turns, 28k событий). Продолжение работы — НОВАЯ сессия в `~/t27`; читать этот файл вместе
с `HANDOVER-20261113-dsh-web-incident.md`. После прочтения — idle / awaiting next ORDER:
НЕ начинать Slice 2–6, PR, push, merge, SEO/GIVEL работы без явного ORDER.

## 1. CLOSED / ACCEPTED

### Milestone: Second Brain Widget — CLOSED
| # | Задача | Статус |
|---|---|---|
| 4 | Интеграция Second Brain Widget | CLOSED (turn 39) |
| 5 | theme.css — изоляция тем, защита генерации | CLOSED (fixture-scoped) |
| 6 | .t27 спецификация для issue #1901 | CLOSED |
| 7 | Task complete #1901 ограничен fixture'ом | CLOSED |

### Milestone: meta format — CLOSED
| # | Задача | Статус |
|---|---|---|
| 14 | Meta format correction (label) | CLOSED (turn 83) |

### Milestone: law overlay / spec-index — CLOSED 2026-11-13 ~20:05
| # | Задача | Статус |
|---|---|---|
| 15 | Law-driven artifact без JS fallback (`spec-index-generated.mjs`, +34 строки, бюджет ≤40 соблюдён) | CLOSED |
| 16 | Law pins `test_calls=159`, `self_loops=0` (`second_brain_index_law.t27:24-25`) | CLOSED, verification PASS |
| 17 | 9 CDP-скриншотов (collapsed/expanded/allow/deny/unknown/source-locations/artifacts-on/artifacts-off/independent-review) | CLOSED — migrated to `~/t27/artifacts/f1d2f458-20261113/screenshots/` + SHA256SUMS |
| 18 | Visual regression before/after | CLOSED — evidence в `artifacts/f1d2f458-20261113/` |
| 19 | Independent review PASS | CLOSED — evidence в `artifacts/f1d2f458-20261113/` |
| 20 | TASK COMPLETE | ACCEPTED |

## 2. OPEN — NEXT SLICES (Milestone 4: миграция spec-index на .t27)

| # | Slice | Scope | Вход | Acceptance | Запрещено | Evidence |
|---|---|---|---|---|---|---|
| 8 | Slice 1: compiler call-site contract | Замена `walkNode`+`fnName` threading на compiler `relations` | PARTIALLY ACCEPTED — classification only | Остаток threading удалён, suite green | Расширять scope за classification | diff + suite output |
| 9 | Slice 2: удалить остатки `walkNode`/`fnName` | NOT STARTED | Slice 1 fully accepted | 0 упоминаний walker в generated-путях | Трогать unrelated specs | grep-zero + suite |
| 10 | Slice 3: сокращение JS orchestrator | NOT STARTED | Slice 2 accepted | Оркестратор только transport | JS computation значений | diff + budget check |
| 11 | Slice 4: удаление ручного fallback | NOT STARTED | Slice 3 accepted | Нет `\|\| LAW.*` fallback | Hand-written значения | node run PASS |
| 12 | Slice 5: explorer policy migration | NOT STARTED | Slice 4 accepted | Политика из .t27 | Новые фичи | suite + seals |
| 13 | Slice 6: visual acceptance | NOT STARTED | Slice 5 accepted | Pixel-parity verdict | Сkip без скриншотов | before/after + verdict |

## 3. BLOCKED / SEPARATE GATES

| # | Задача | Статус | Условие снятия |
|---|---|---|---|
| 21 | PR | NOT STARTED — SEPARATE GATES | явный ORDER |
| 22 | Push | NOT STARTED — SEPARATE GATES | PR принят |
| 23 | Merge | NOT STARTED — SEPARATE GATES | CI green |
| 24 | **Artifact migration**: найти/пересоздать 9 скриншотов + visual-regression diff + review evidence; перенести в постоянный store; сохранить hashes, timestamps, paths; обновить статусы #17-19 на CLOSED | **OPEN — PRIORITY 1** (`/tmp` чистится ОС; уже дважды пропадали) | файлы в постоянном store |
| 25 | Performance optimization | OPEN — BASELINE REQUIRED (сначала замер, потом оптимизация) | baseline зафиксирован |
| 26 | SEO / GIVEL recovery: missing SEO data, wrong site/GIVEL info, slow load | OPEN — INCIDENT | отдельный ORDER, не смешивать со slices |
| 27 | Domain cutover | BLOCKED — CLIENT DNS | клиент обновит DNS |
| 28 | «Продолжайте распространять обновления» | OPEN — не финализировано в логе | уточнение владельца |

## 4. KNOWN INCIDENTS

- **Инцидент 3081 (2026-11-13):** эта сессия выполняла bash «kill dsh web processes» и
  SIGKILL'ила собственный host-процесс; launchd KeepAlive перезапускал, вкладки клиента
  вешались (7 PID за вечер). Полный разбор: `HANDOVER-20261113-dsh-web-incident.md`.
- **`test_cells` vs `test_calls`** — опечатка в отчёте агента, НЕ баг кода. Код верифицирован
  (PASS: test_calls===159, self_loops===0) — не «перепроверять» по сто раз.
- **Скриншоты пропадали из /tmp дважды** и пересоздавались — причина приоритета задачи #24.

## 5. WORKTREE / GIT SAFETY

- Worktree: `~/t27/.agents/worktrees/dsh-f1d2f458-761f/.agents/worktrees/t27ai-bot-mvp`
  — **вложенный worktree**: перед любой мутацией `git status` + проверить, тот ли это каталог.
- Один агент — один worktree — один набор путей записи; конфликт → стоп мутаций,
  передать владельцу milestone.
- Не смешивать: shim / spec / SEO / GIVEL — отдельные задачи, отдельные ветки.

## 6. SAFETY RULES (обязательно)

- НЕ resume сессию `f1d2f458` без guard (см. `GUARD-ISSUE-DRAFT-dsh-web-self-kill.md`).
- ЗАПРЕЩЕНО: `pkill`/`killall` по broad pattern; SIGKILL host/web процессов; kill dsh web processes.
- Управляемый рестарт сервиса — только `launchctl kickstart -k gui/501/com.t27os.dsh-web`.
- Токен 3081 всегда свежий в последней строке `dsh web: http://...` из
  `~/Library/Logs/t27os/dsh-web.out.log`.
- 3080 fallback (`ai.t27.dsh-web`) и MCP 9010 не трогать.
