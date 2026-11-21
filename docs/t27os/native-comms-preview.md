# Native T27 communications preview

This is a separately installable arm64 macOS communications preview built from
`.t27`. It opens the existing native `lu_open_mac` controller directly. It does
not load or replace the Carbon instrument dashboard, account gateway, agent
panes or an existing T27 OS installation.

The visible and bundle marketing version is **0.3.0**. The current development
bundle build is numeric **19**. This is a development build, not a notarized or
App Store release.

## What this candidate does

- Stores this installation's P-256 private identity in its own device-local
  Apple Keychain item. The private key is not passed through commands, logs,
  invitations or the owner service.
- Copies a `t27p1:` PUBLIC card using the host-only pasteboard. The recipient
  pastes it into the hidden friend field. A display label is not authentication.
- Registers at an explicitly configured owner's HTTPS service using signed
  nonce/session/expiry proofs. Lookup is by the full public key.
- Tries bounded, signed IPv4/global-IPv6 **host** candidates. Directions and
  verified source candidates are admitted independently. It automatically uses
  an opaque HTTPS relay when direct probing fails. There is no normal LAN versus
  Internet route selector.
- Requires both devices to compare the actual nonzero six-digit codes and
  explicitly confirm before text or media. A new/replaced identity is not
  silently trusted. The service cannot substitute a peer signing key.
- Keeps a message PENDING until peer acknowledgement and READ until the visible
  recipient conversation commits its read receipt. Saving must succeed before a
  message is persistently queued; histories are scoped to the selected friend.
- Supports an explicit listening-only call: it creates a real output AudioQueue
  without an input queue. Transport silence does not increment capture counters.
  This is distinct from mute and from a synthetic audio fixture.

The owner service has bounded registrations, pair-authorized rooms, participant
leases, request/packet limits and atomic queue admission. Poll batches remain
available until a signed recipient acknowledgement. It neither decrypts content
nor forwards arbitrary IP traffic. It sees public identities, candidate/connection
addresses, timing and opaque encrypted frames; it is not a metadata-hiding system.

## Current limits

A common operator HTTPS endpoint must be configured on both installations before
pasting a friend card. No permanent public endpoint is shipped or assumed. An
operator change or an expired temporary tunnel needs a current operator setting.

Host interface candidates are not IPv4 NAT mappings. An owner-run UDP reflector,
nearby/offline resolver, IPv4 NAT traversal proof, QR invitation and fuller local
profile UX remain follow-up work. A Quick Tunnel is HTTPS ingress, not public UDP
or STUN. No third-party DHT or Tailscale installation is used by this preview.

On 2026-11-21, the same signed build 19 ran on two Macs on distinct networks
(cellular hotspot and home Wi-Fi). Direct public-IPv6 UDP and forced HTTPS relay
both delivered fresh messages in both directions with READ receipts. Calls
reached ACTIVE and normal End in each originating direction. The direct call
produced nonzero playback on the listening-only peer; the relay call accepted
audio frames with active output, but its nonzero-playback counter stayed zero.
These are native button-handler QA-dispatcher results, not a manual audible
speech test. Two live microphones, human speech intelligibility, IPv4 NAT
traversal and automatic handover on a live failure remain unverified. Normal
microphone permission prompts must be answered by the device owner.

Video controls remain experimental. The current relay TEST-video attempt
reported zero remote render serials, and live camera was not reverified in this
candidate. Prior video results are not represented as results of this binary.
Do not use video as an acceptance claim for this change. The preview ends audio
on window deactivation; background/closed-app calling is not provided.

## Getting the compiler tools

`t27b` and `t27c` are not assumed to be installed on PATH. From this checkout,
with the repository's Rust/Cargo bootstrap prerequisites installed, build them
sequentially (no running Simulator or concurrent heavy build):

```sh
cargo build --release -p t27c
cargo build --release -p t27b
export PATH="$PWD/target/release:$PATH"
command -v t27c
command -v t27b
```

The tested native compiler binary SHA-256 was
`99ce538a63d2f551655863dcd42fef0eb83174e4f958aabd21b261f976a91777`.
The application was tested with that previously built compiler; rebuilding the
bootstrap at a different revision is a separate toolchain validation, not a
claim of byte-identical output. No compiler executable or private certificate
is embedded in this source patch.

## Source boundary

| Component | Source |
| --- | --- |
| Dedicated entry and bundle data | `specs/t27os/local_comms_mac.t27`, `local_comms_bundle.t27` |
| Normal AppKit controller | `specs/t27os/local_comms_ui.t27` |
| Identity, durable text and call adapter | `specs/t27os/local_comms_bridge.t27`, `local_identity.t27` |
| Authenticated direct/HTTPS transport | `specs/t27os/directory_native.t27`, `directory_wire.t27`, `directory_candidates.t27` |
| Existing sealed call/text machinery | `specs/t27os/call_*.t27`, `chat_*.t27`, `peer_identity*.t27` and their imports |
| Linux owner service | `specs/t27os/owner_directory/*.t27` |

Existing master SDK and utility modules are reused. Five existing
`keysync_host.t27` helpers only become public; their bodies are not replaced.
The historical call donor is provenance, not a merged release or approval:
commit `858e2014`, from closed, unmerged PR #12035. This preview does not reopen
that PR or import its Carbon dashboard and unrelated agent changes.

The Apple media path still uses the existing CommonCrypto GCM SPI and libffi.
That is a development implementation, not an App Store compatibility claim.
The Linux verifier uses the public OpenSSL P-256/SHA-256 APIs and X9.62 DER
signatures; no new cryptographic algorithm is implemented here.

## Build the Mac app

Tested: Apple Silicon, macOS 27.2.1, Xcode/SDK 27.2, the real `t27b --macho`
backend. `t27c gen` stubs of host calls are not an app build. Use a local APFS
staging directory for code signing; keep generated files under the build root.
Run these commands at this checkout's root, with your installed `t27b` on PATH:

```sh
COMMS_BUILD="$PWD/target/native-comms"
COMMS_APP="$COMMS_BUILD/T27OSCommunications.app"
COMMS_SDK="$(xcrun --sdk macosx --show-sdk-path)"
COMMS_SDK_VERSION="$(xcrun --sdk macosx --show-sdk-version)"
mkdir -p "$COMMS_APP/Contents/MacOS"
t27b build specs/t27os/local_comms_mac.t27 --macho -o "$COMMS_BUILD/entry.o"
t27b build specs/t27os/local_comms_bundle.t27 --macho -o "$COMMS_BUILD/bundle.o"
t27b build specs/t27os/local_comms_ui.t27 --macho -o "$COMMS_BUILD/ui.o"
t27b build specs/t27os/local_comms_bridge.t27 --macho -o "$COMMS_BUILD/bridge.o"
t27b build specs/t27os/directory_native.t27 --macho -o "$COMMS_BUILD/directory.o"
xcrun ld -arch arm64 -platform_version macos 11.0 "$COMMS_SDK_VERSION" \
  -syslibroot "$COMMS_SDK" -e _main -lSystem \
  -o "$COMMS_BUILD/bundle-tool" "$COMMS_BUILD/bundle.o"
"$COMMS_BUILD/bundle-tool" > "$COMMS_APP/Contents/Info.plist"
plutil -lint "$COMMS_APP/Contents/Info.plist"

cp "$COMMS_BUILD/ui.o" "$COMMS_BUILD/ui.local.o"
cp "$COMMS_BUILD/bridge.o" "$COMMS_BUILD/bridge.local.o"
cp "$COMMS_BUILD/directory.o" "$COMMS_BUILD/directory.local.o"
nm -gU "$COMMS_BUILD/ui.o" | awk '$NF ~ /^_lu_/ {print $NF}' > "$COMMS_BUILD/ui.exports"
nm -gU "$COMMS_BUILD/bridge.o" | awk '$NF ~ /^(_sc_|_vd_|_cbp_|_csg_fout|_cv_|_cvl_|_vte_out|_vtd_out)/ {print $NF}' > "$COMMS_BUILD/bridge.exports"
nm -gU "$COMMS_BUILD/directory.o" | awk '$NF ~ /^_dn_/ {print $NF}' > "$COMMS_BUILD/directory.exports"
nmedit -s "$COMMS_BUILD/ui.exports" "$COMMS_BUILD/ui.local.o"
nmedit -s "$COMMS_BUILD/bridge.exports" "$COMMS_BUILD/bridge.local.o"
nmedit -s "$COMMS_BUILD/directory.exports" "$COMMS_BUILD/directory.local.o"
xcrun ld -arch arm64 -platform_version macos 11.0 "$COMMS_SDK_VERSION" \
  -headerpad 0x1000 -syslibroot "$COMMS_SDK" -e _main \
  -lSystem -lobjc -lffi -export_dynamic \
  -framework Foundation -framework AppKit -framework CoreGraphics \
  -framework CoreText -framework CoreFoundation -framework QuartzCore \
  -framework Security -framework ImageIO -framework AudioToolbox \
  -framework AVFoundation -framework CoreMedia -framework CoreVideo \
  -framework VideoToolbox -framework IOSurface \
  -o "$COMMS_APP/Contents/MacOS/T27OS" "$COMMS_BUILD/entry.o" \
  "$COMMS_BUILD/ui.local.o" "$COMMS_BUILD/bridge.local.o" \
  "$COMMS_BUILD/directory.local.o"
codesign --force --sign "${COMMS_SIGN_IDENTITY:--}" "$COMMS_APP"
codesign --verify --deep --strict "$COMMS_APP"
```

The default is local ad-hoc signing. Use your own signing identity when needed;
no certificate, provisioning secret or private identity is supplied by this
source tree. Existing installed apps are not replaced by these commands.
Double-click `T27OSCommunications.app`, or use `open "$COMMS_APP"`.

## Build and run the owner service

Prerequisites: `t27c`, Zig supporting `aarch64-linux-gnu.2.39`, OpenSSL 3 public
headers and an arm64 Ubuntu 24.04 image with `libcrypto.so.3`. The actual tested
cached image ID/digest was
`sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3`.
Tests use `--pull=never` and no network for containers that run unit suites.

Generate each owner-service source with `t27c gen-c`; never hand-edit the output.
The compiler emits function prototypes before definitions. Extract those emitted
prototype lines verbatim as cross-module headers. Compile separate objects for
`directory_crypto`, `directory_protocol`, `directory_health` and
`directory_http`; link them with `directory_host.c`. Use the installed SDK
headers for `socket`, `bind`, `accept`, `recv`, `send`, `getrandom`,
`gettimeofday` and OpenSSL. The host entry is `dt_owner_run`: compile the product
with `-Ddt_owner_run=main`, and the tests without that rename and with
`-DT27_TEST_MAIN`. This avoids editing generated code or colliding with the
generated test entry.

The exact executable build recipe is in
[`native-comms-owner-build.md`](native-comms-owner-build.md). After building,
this single foreground command starts the bounded service from the checkout:

```sh
docker run --rm --pull=never --name t27-owner-directory \
  --read-only --cap-drop ALL --security-opt no-new-privileges \
  -p 127.0.0.1:49300:49300 \
  --mount "type=bind,src=$PWD/target/comms-owner/build,dst=/opt/t27,readonly" \
  ubuntu:24.04 /opt/t27/t27-directory
```

There is no required custom Docker network. An operator must deliberately place
normal trusted HTTPS in front of this loopback service for two remote apps.
A disposable Quick Tunnel can exercise that ingress without an account; it is
not the shipped default, permanent deployment, or a UDP mapping service. Never
put its expired URL in a shared invitation or installation recipe.

## Normal joint test

1. Run one app per Mac. Configure the same current owner HTTPS service in each.
2. Copy each PUBLIC card and paste the nominated other device's card. Select
   Pair, Connect chat, Answer, then independently compare and confirm both codes.
3. Send two distinct test messages in opposite directions. Check the recipient
   text and the sender's READ, not just a server HTTP status.
4. Test a call in each originating direction. With an undetermined microphone,
   explicitly choose listening-only. Allow normal owner-controlled microphone
   permission before claiming two-microphone duplex.
5. Mute and End. Confirm both terminal reasons and stopped capture. Repeat after
   Codes differ, with fresh codes, without treating a restart as Redial.
6. Keep different-network, human audibility and video checks separately labelled.
   Do not save live camera images or recordings as evidence.

## Automated and compiled checks

```sh
t27b test specs/t27os/local_comms_mac.t27
t27b test specs/t27os/local_comms_bundle.t27
t27b test specs/t27os/directory_native.t27
t27b test specs/t27os/directory_candidates.t27
t27b test specs/t27os/call_session.t27
```

The frozen build-19 sources ran 57 tests on 2026-11-20 across these five specifications. The Linux
C backend executed 13 tests: crypto 1, protocol 7, health 2, HTTP framing 2 and
host policy 1. The generic Zig `test-report` for the Linux SDK host is BLOCKED on
undeclared POSIX SDK names; it is not counted as a pass. The actual generated C
executables are the supported host verification route.

The compiled Mac `comms_directory_probe.t27` checks actual Apple-to-Linux
signatures, wrong principals, consumed/replayed nonces, leases, signed candidate
proofs, wrong room participant, retained poll/ACK and logout. It uses synthetic
in-memory identities, loopback only and emits no private key or raw card.
`comms_direct_probe_body.t27` is a QA companion for the unchanged native transport:
its generated concatenation tests real loopback UDP, encrypted text, asymmetric
fallback and End. Set `T27_COMMS_QA_HOME` to an explicit dedicated absolute test
directory. Neither fixture is the two-physical-Mac evidence.

Two-Mac runs use an explicitly opted-in `--t27-local-qa` dispatcher that invokes
the same native button handlers. Its public-card-only helper transfers nominated
PUBLIC bytes out of band; it is not a content/media tunnel. Exact code equality
is checked before explicit confirmations. The M4 normal screen was also checked
through native accessibility. The M1 Orca AX helper was denied permission; that
limitation is not represented as a native app failure or an unperformed click.

## Compiled integration probes

With the owner service running locally on port 49300, the protocol probe is:

```sh
t27b build specs/t27os/comms_directory_probe.t27 --macho -o "$COMMS_BUILD/protocol-probe.o"
xcrun ld -arch arm64 -platform_version macos 11.0 "$COMMS_SDK_VERSION" \
  -syslibroot "$COMMS_SDK" -e _main -lSystem -lobjc \
  -framework Foundation -framework Security \
  -o "$COMMS_BUILD/protocol-probe" "$COMMS_BUILD/protocol-probe.o"
"$COMMS_BUILD/protocol-probe"
```

The direct companion must be concatenated with `directory_native.t27` into a
new generated `.t27` file in a generated specs/t27os directory. Its sibling
imports must resolve to this checkout's specifications, never a sealed donor
snapshot. Link its object with the localized bridge object from the app recipe,
keeping `_main` and `_dn_` exports. Set `T27_COMMS_QA_HOME` to a dedicated writable
absolute directory before running. It reports four actual transport scenarios,
not an unconditional unit-test marker. Current compiled results were exit 0:
9 protocol scenarios and 4 direct scenarios.
