# HANDOVER: инцидент dsh-web 3081, 2026-11-13

## Состояние
- 3081 поднят launchd-сервисом `com.t27os.dsh-web`, один владелец, PID см. `launchctl print gui/501/com.t27os.dsh-web`.
- 3080 fallback = `ai.t27.dsh-web` (PID стартовал 20:00) — не трогать.
- Свежий токен всегда в `~/Library/Logs/t27os/dsh-web.out.log` (строка `dsh web: http://127.0.0.1:3081/?token=...`).

## Что произошло (root cause)
- Сессия `session-f1d2f458-eb2b-4d03-a1d3-92868ba7bbe3` (задача «почему бот не работает? изучи @t27ai_bot», workdir `~/t27`, worktree `.agents/worktrees/dsh-f1d2f458-761f`) в attempt починить скриншоты/CDP выполняла bash «kill dsh web processes» и `killall "BrowserOS neo"`, SIGKILL'я собственный host-процесс на 3081.
- KeepAlive launchd перезапускал; интерактивная browser-вкладка (не cron — интервалы нерегулярны: 0м…892м) продолжала слать «дальше», сессия возобновлялась, цикл повторялся (7 PID за вечер).
- Полный лог: `~/.dsh/sessions/quarantine-20261113/session-f1d2f458-eb2b-4d03-a1d3-92868ba7bbe3/session.v4.jsonl.zstd` (19.6 МБ, сохранён).

## ЗАПРЕЩЕНО (обязательно)
- НЕ resume и не открывать сессию `f1d2f458`.
- НЕ выполнять `killall node`, `pkill -f web`, `pkill -f dsh` и любые broad-pattern kill.
- НЕ перезапускать сервис вручную без причины: единственный управляемый restart —
  `launchctl kickstart -k gui/501/com.t27os.dsh-web`.
- НЕ трогать: MCP 9010, 3080 fallback (PID 826), launchd-определения `com.t27os.dsh-web` и `ai.t27.dsh-web`.

## Продолжение работы сессии f1d2f458 (задача t27ai_bot)
- Milestone `law overlay / spec-index` — TASK COMPLETE (условно: независимый review фактически проверил код и 9 screenshots).
- Дальше по плану были PR / push / merge — отдельные gates.
- Рабочее дерево: `~/t27/.agents/worktrees/dsh-f1d2f458-761f/.agents/worktrees/t27ai-bot-mvp` (вложенный worktree — проверить состояние git перед продолжением).
- Начинать НОВУЮ сессию в `~/t27` с этим файлом как контекстом.
