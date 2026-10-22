# T27 OS: one installer, built with make (#11044), and the developer shortcuts below it. Thin on
# purpose: the install decisions (platform, prefix, version pin, idempotence, uninstall list, dry run,
# opt-in flags) are specs/t27os/install.t27; this file names the same files and flags and carries
# them out with cargo and install(1). Not wired into CI (the required checks are the ruleset's answer:
# gh api repos/gHashTag/t27/rules/branches/master). Anchor: phi^2 + phi^-2 = 3

PREFIX ?= /usr/local
T27B := $(filter arm64 aarch64,$(shell uname -m))
LIST := $(PREFIX)/share/t27/install.list
PAIRS := scripts/tri:bin/tri target/release/t27c:bin/t27c target/release/tri:target/release/tri target/release/t27c:target/release/t27c $(if $(T27B),target/release/t27b:bin/t27b) $(if $(filter on,$(CLAW)),$(foreach f,specs/claw/service.t27 specs/claw/text.t27 .trinity/seals/claw_ClawService.json .trinity/seals/claw_ClawText.json,$(f):share/t27/claw/$(f)))

# The app target (#11315). specs/t27os/install.t27 decides the products, the frameworks, the plist text, the
# signing and the refusals; this names the same files and flags. T27OS_BUILD is the commit count.
T27OS_OUT := target/t27os
T27OS_BUILD ?= $(shell git rev-list --count HEAD)
T27OS_T27B := $(or $(CARGO_TARGET_DIR),target)/release/t27b
T27OS_FRAMEWORKS := Foundation AppKit CoreGraphics CoreText CoreFoundation QuartzCore Security WebKit ImageIO
T27OS_MIN_MACOS := 11.0

.PHONY: help install uninstall t27os verify seal-check seal warnings-baseline

## make install                       build and install tri, t27c and t27b under PREFIX (/usr/local); PIN=<version> insists on that t27c
## make -n install                    print the plan and change nothing
## make install PREFIX=$HOME/.local   install under a directory you own
## make install CLAW=on               also install TRI CLAW; EARN=on, the lab node, is refused
## make uninstall                     remove exactly the files the install listed
## tri --version                      check an install, from PREFIX/bin
## make t27os                        build target/t27os/T27OS.app and target/t27os/t27os-pty (Apple silicon; ad hoc signed, or T27OS_SIGN_ID=<identity>)
## make help                          this list; advisory shortcuts: make seal-check | seal | verify | warnings-baseline
help:
	@sed -n 's/^## /  /p' $(MAKEFILE_LIST)

install:
	@case "$$(uname -s)-$$(uname -m)" in Darwin-arm64|Darwin-x86_64|Linux-aarch64|Linux-x86_64) ;; *) echo "unsupported platform: $$(uname -s) $$(uname -m)" >&2; exit 2;; esac
	@case "$(PREFIX)" in *" "*|*..*) ;; /?*) exit 0;; esac; echo "PREFIX must be an absolute path, no spaces, no ..: $(PREFIX)" >&2; exit 2
	@test "$(EARN)" != on || { echo "EARN=on: no lab node ships in this checkout yet; nothing was installed" >&2; exit 2; }
	./scripts/tri disk
	cargo build --release -p t27c -p tri $(if $(T27B),-p t27b)
	@test -z "$(PIN)" || test "$$(target/release/t27c --version | head -1)" = "t27c $(PIN)" || { echo "version pin $(PIN) not met: $$(target/release/t27c --version | head -1)" >&2; exit 2; }
	@set -e; for p in $(PAIRS); do s=$${p%%:*}; d=$(PREFIX)/$${p#*:}; mkdir -p "$$(dirname "$$d")"; m=755; case "$$s" in specs/*|.trinity/*) m=644;; esac; install -C -m $$m "$$s" "$$d"; done
	@set -e; mkdir -p $(PREFIX)/share/t27; m=$$({ cat $(LIST) 2>/dev/null || :; printf '%s\n' $(PAIRS) | sed 's/^[^:]*://'; echo share/t27/install.list; } | sort -u); [ "$$m" = "$$(cat $(LIST) 2>/dev/null)" ] || printf '%s\n' "$$m" > $(LIST)
	$(if $(filter on,$(CLAW)),@cd $(PREFIX)/share/t27/claw && $(PREFIX)/bin/t27c seal --verify specs/claw/service.t27)

uninstall:
	@if [ ! -f $(LIST) ]; then echo "nothing to remove: no $(LIST)"; elif grep -qE '^/|\.\.' $(LIST); then echo "$(LIST) names a path outside PREFIX; nothing removed" >&2; exit 2; else cd $(PREFIX) && xargs rm -f < share/t27/install.list && rmdir share/t27/claw/specs/claw share/t27/claw/specs share/t27/claw/.trinity/seals share/t27/claw/.trinity share/t27/claw share/t27 target/release target 2>/dev/null; true; fi

t27os:
	@test "$$(uname -s)-$$(uname -m)" = Darwin-arm64 || { echo "make t27os: Apple silicon macOS only (app_platform_ok, specs/t27os/install.t27)" >&2; exit 2; }
	@case "$(T27OS_BUILD)" in ""|0*|*[!0-9]*) echo "make t27os: T27OS_BUILD must be a commit count, got '$(T27OS_BUILD)'" >&2; exit 2;; esac
	cargo build --release -p t27b
	@set -e; O=$(T27OS_OUT); A=$$O/T27OS.app/Contents; mkdir -p $$A/MacOS; \
	$(T27OS_T27B) build specs/t27os/pty_server.t27 -o $$O/pty_server.o --macho; \
	$(T27OS_T27B) build specs/t27os/glass_window.t27 -o $$O/glass_window.o --macho; \
	$(T27OS_T27B) build specs/t27os/plist_main.t27 -o $$O/plist_main.o --macho; \
	for f in pty_server glass_window; do n=$$(nm -u $$O/$$f.o | grep -c .); test $$n -gt 0 || { echo "make t27os: $$f.o has no undefined symbols: this t27b has no @extern (object_ok)" >&2; exit 2; }; done; \
	SDK=$$(xcrun --show-sdk-path); SDKV=$$(xcrun --show-sdk-version); \
	LD="ld -arch arm64 -platform_version macos $(T27OS_MIN_MACOS) $$SDKV -syslibroot $$SDK -e _main -lSystem"; \
	$$LD -o $$O/t27os-pty $$O/pty_server.o; \
	$$LD -o $$O/plist_main $$O/plist_main.o; \
	$$LD -lobjc $(foreach f,$(T27OS_FRAMEWORKS),-framework $(f)) -o $$A/MacOS/T27OS $$O/glass_window.o; \
	T27OS_BUILD=$(T27OS_BUILD) $$O/plist_main > $$A/Info.plist; plutil -lint $$A/Info.plist >/dev/null; \
	ID="$${T27OS_SIGN_ID:--}"; codesign --force --sign "$$ID" $$O/T27OS.app; codesign --force --sign "$$ID" $$O/t27os-pty; \
	echo "built $$O/T27OS.app and $$O/t27os-pty (build $(T27OS_BUILD), signed with $$ID)"

# Advisory umbrella: seal freshness, warnings baseline and a quick test; always exits 0.
verify:
	@scripts/verify.sh

# Advisory: NMSE seal freshness. Exit 0 fresh / 2 stale / 3 unsealed. Never reseals.
seal-check:
	@scripts/reseal-check.sh

# Explicit, reviewed reseal of the NMSE seal; prompts, or RESEAL_YES=1.
seal:
	@scripts/reseal-apply.sh

# Advisory: non-test build warnings vs the recorded baseline (#969). Never edits code.
warnings-baseline:
	@scripts/warnings-baseline.sh
