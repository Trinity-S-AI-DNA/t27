---
name: t27os-terminal-input-memory
description: Точные исходники T27OS для исправления input field - term_input.t27
metadata:
  type: project
---

# T27OS Terminal Input Memory

## Найденные исходники

### Основной файл: term_input.t27
**Путь**: `.claude/worktrees/idle-cpu-11419/specs/t27os/term_input.t27`

Это главный файл, отвечающий за обработку ввода в терминале T27OS.

### Ключевые компоненты term_input.t27:

1. **Обработка клавиш** (tk_key_bytes function)
   - Обработка Enter, Tab, Backspace, Escape
   - Обработка стрелок (вкл/выкл режим application cursor keys)
   - Обработка Command комбинаций (Cmd+C, Cmd+V, Cmd+A)
   - Обработка F1-F12 функциональных клавиш

2. **Командные клавиши** (tk_cmd_route function)
   - Copy (Cmd+C)
   - Paste (Cmd+V) 
   - Select All (Cmd+A)
   - Line keys (Cmd+Left, Cmd+Right, Cmd+Backspace)

3. **Вставка текста** (ti_paste function)
   - Bracketed paste (ESC [ 200 ~ ... ESC [ 201 ~)
   - Raw paste с конвертацией LF в CR

4. **Копирование текста** (ti_sel_text function)
   - Выделенный текст как UTF-8
   - Trim trailing spaces
   - Rows joined by LF

### Проблемы, которые нужно исправить:

1. **Enter не работает properly**
   - Сейчас: Enter sends CR (13)
   - Нужно: Обработка Enter для чата должна отправлять сообщения

2. **Нет auto-resize для input field**
   - Нужно: Добавить автоматическое изменение размера input

3. **Нет proper focus management**
   - Нужно: Автоматический фокус при клике

4. **Нет IME support**
   - Нужно: Добавление compositionstart/end handlers

### Команды для создания hot-reload:

```bash
# Компиляция T27OS
cd bootstrap && cargo build --release

# Генерация из .t27 спецификаций
tri gen specs/t27os/term_input.t27

# Тестирование
tri test specs/t27os/term_input.t27

# Verdict
tri verdict
```

### Связанные файлы:

- `specs/t27os/vt.t27` - Virtual Terminal implementation
- `specs/t27os/term_pane.t27` - Terminal pane geometry
- `specs/t27os/glass_window.t27` - NSEvents forwarding

### Контекст использования:
T27OS terminal используется в Trinity Orchestrator для чата с AI агентами, включая Queen и другими агентами системы. Input field должен работать идеально для корректной коммуникации.

## Действия для исправления:

1. Использовать term_input.t27 как основу
2. Добавить обработку для чат-инпута (Enter для отправки)
3. Добавить auto-resize functionality
4. Улучшить UX с focus management
5. Сохранить все существующие терминальные функции

Связано с: [[t27os-terminal-input-memory]]