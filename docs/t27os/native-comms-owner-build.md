# Owner directory: generated Linux build

The service source is `specs/t27os/owner_directory/`. It is generated with t27c;
none of the emitted C is edited. It is an authenticated directory and bounded
opaque pair relay, not an open IP proxy. The client requires a current operator
HTTPS URL. The command below exposes only loopback HTTP; terminate TLS with your
own operator frontend. No permanent public deployment is supplied.

## Prerequisites

Use the t27c bootstrap from the same checkout (see the client guide), Zig cc,
OpenSSL development headers and an arm64 Ubuntu 24.04 runtime with libcrypto.so.3.
The tested runtime is cached image ubuntu:24.04, arm64 image ID
`sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3`.
The test commands use `--pull=never`: they do not install packages or pull images.
Tested headers were OpenSSL 3.6; the runtime ABI was OpenSSL 3.0.
A different local image must be checked for architecture and the library first.

Run from the checkout root. Define absolute executable/header paths:

```sh
export T27C="$PWD/target/release/t27c"
export ZIG="/path/to/zig"
export OPENSSL_INCLUDE="/path/to/openssl/include"
export OWNER_BUILD="$PWD/target/comms-owner"
mkdir -p "$OWNER_BUILD/gen/c" "$OWNER_BUILD/gen/headers" "$OWNER_BUILD/build" "$OWNER_BUILD/deps"
export ZIG_LOCAL_CACHE_DIR="$OWNER_BUILD/build/zig-local"
export ZIG_GLOBAL_CACHE_DIR="$OWNER_BUILD/build/zig-global"
image_container=$(docker create --pull=never ubuntu:24.04)
docker cp "$image_container:/usr/lib/aarch64-linux-gnu/libcrypto.so.3" "$OWNER_BUILD/deps/libcrypto.so.3"
docker rm "$image_container"
ln -sf libcrypto.so.3 "$OWNER_BUILD/deps/libcrypto.so"
```

## Exact generation and SDK linking sequence

The prototype headers below are extracted verbatim from generated definitions.
They contain no authored behavior. The host's `dt_owner_run` entry is mapped to
`main` only for the product link. Test executables instead use T27_TEST_MAIN.
The `.2.39` target is deliberate: the runtime library needs newer glibc symbols
than Zig's default Linux target.

```sh
"$T27C" gen-c "$PWD/specs/t27os/owner_directory/directory_health.t27" > "$OWNER_BUILD/gen/c/directory_health.c"
"$T27C" gen-c "$PWD/specs/t27os/owner_directory/directory_crypto.t27" > "$OWNER_BUILD/gen/c/directory_crypto.c"
"$T27C" gen-c "$PWD/specs/t27os/owner_directory/directory_protocol.t27" > "$OWNER_BUILD/gen/c/directory_protocol.c"
"$T27C" gen-c "$PWD/specs/t27os/owner_directory/directory_host.t27" > "$OWNER_BUILD/gen/c/directory_host.c"
"$T27C" gen-c "$PWD/specs/t27os/owner_directory/directory_http.t27" > "$OWNER_BUILD/gen/c/directory_http.c"
"$T27C" gen-c "$PWD/specs/t27os/owner_directory/directory_crypto_probe.t27" > "$OWNER_BUILD/gen/c/directory_crypto_probe.c"
for module in directory_crypto directory_protocol directory_health directory_http; do
  sed -nE 's/^(bool|int64_t|uint64_t|uint32_t|uint16_t|uint8_t|int32_t|void|size_t|double|float|int|char \*) ([A-Za-z_][A-Za-z_0-9]*\(.*\)) \{$/\1 \2;/p' "$OWNER_BUILD/gen/c/$module.c" > "$OWNER_BUILD/gen/headers/$module.h"
done
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -include openssl/ec.h -include openssl/sha.h -c "$OWNER_BUILD/gen/c/directory_crypto.c" -o "$OWNER_BUILD/build/directory_crypto.o"
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -include openssl/ec.h -include openssl/sha.h -DT27_TEST_MAIN "$OWNER_BUILD/gen/c/directory_crypto.c" "-L$OWNER_BUILD/deps" -lcrypto -o "$OWNER_BUILD/build/directory_crypto-tests"
docker run --rm --pull=never --network none --read-only --mount "type=bind,src=$OWNER_BUILD/build,dst=/opt/t27,readonly" ubuntu:24.04 /opt/t27/directory_crypto-tests
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -include openssl/sha.h -include "$OWNER_BUILD/gen/headers/directory_crypto.h" -c "$OWNER_BUILD/gen/c/directory_protocol.c" -o "$OWNER_BUILD/build/directory_protocol.o"
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -include openssl/sha.h -include "$OWNER_BUILD/gen/headers/directory_crypto.h" -DT27_TEST_MAIN "$OWNER_BUILD/gen/c/directory_protocol.c" "$OWNER_BUILD/build/directory_crypto.o" "-L$OWNER_BUILD/deps" -lcrypto -o "$OWNER_BUILD/build/directory_protocol-tests"
docker run --rm --pull=never --network none --read-only --mount "type=bind,src=$OWNER_BUILD/build,dst=/opt/t27,readonly" ubuntu:24.04 /opt/t27/directory_protocol-tests
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -c "$OWNER_BUILD/gen/c/directory_health.c" -o "$OWNER_BUILD/build/directory_health.o"
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -DT27_TEST_MAIN "$OWNER_BUILD/gen/c/directory_health.c" -o "$OWNER_BUILD/build/directory_health-tests"
docker run --rm --pull=never --network none --read-only --mount "type=bind,src=$OWNER_BUILD/build,dst=/opt/t27,readonly" ubuntu:24.04 /opt/t27/directory_health-tests
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -c "$OWNER_BUILD/gen/c/directory_http.c" -o "$OWNER_BUILD/build/directory_http.o"
"$ZIG" cc -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -DT27_TEST_MAIN "$OWNER_BUILD/gen/c/directory_http.c" -o "$OWNER_BUILD/build/directory_http-tests"
docker run --rm --pull=never --network none --read-only --mount "type=bind,src=$OWNER_BUILD/build,dst=/opt/t27,readonly" ubuntu:24.04 /opt/t27/directory_http-tests
"$ZIG" cc -Ddt_owner_run=main -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -include sys/socket.h -include netinet/in.h -include unistd.h -include sys/time.h -include sys/random.h -include "$OWNER_BUILD/gen/headers/directory_health.h" -include "$OWNER_BUILD/gen/headers/directory_http.h" -include "$OWNER_BUILD/gen/headers/directory_protocol.h" "$OWNER_BUILD/gen/c/directory_host.c" "$OWNER_BUILD/build/directory_crypto.o" "$OWNER_BUILD/build/directory_protocol.o" "$OWNER_BUILD/build/directory_health.o" "$OWNER_BUILD/build/directory_http.o" "-L$OWNER_BUILD/deps" -lcrypto -o "$OWNER_BUILD/build/t27-directory"
"$ZIG" cc -DT27_TEST_MAIN -target aarch64-linux-gnu.2.39 -O2 -Wno-incompatible-pointer-types -Wno-int-conversion -Wno-deprecated-declarations "-I$OPENSSL_INCLUDE" -include stdbool.h -include stdint.h -include sys/socket.h -include netinet/in.h -include unistd.h -include sys/time.h -include sys/random.h -include "$OWNER_BUILD/gen/headers/directory_health.h" -include "$OWNER_BUILD/gen/headers/directory_http.h" -include "$OWNER_BUILD/gen/headers/directory_protocol.h" "$OWNER_BUILD/gen/c/directory_host.c" "$OWNER_BUILD/build/directory_crypto.o" "$OWNER_BUILD/build/directory_protocol.o" "$OWNER_BUILD/build/directory_health.o" "$OWNER_BUILD/build/directory_http.o" "-L$OWNER_BUILD/deps" -lcrypto -o "$OWNER_BUILD/build/directory_host-tests"
docker run --rm --pull=never --network none --read-only --mount "type=bind,src=$OWNER_BUILD/build,dst=/opt/t27,readonly" ubuntu:24.04 /opt/t27/directory_host-tests
```

Expected Linux unit counts: crypto 1, protocol 7, health 2, HTTP 2, host 1.
The generic Zig host test-report rejects undeclared POSIX SDK functions; it is
not an alternative PASS receipt for these generated-C SDK tests.

## Run the resulting service

This command works from any directory after setting OWNER_BUILD above:

```sh
docker run --rm --pull=never --name t27-owner-directory \
  --read-only --cap-drop ALL --security-opt no-new-privileges \
  -p 127.0.0.1:49300:49300 \
  --mount "type=bind,src=$OWNER_BUILD/build,dst=/opt/t27,readonly" \
  ubuntu:24.04 /opt/t27/t27-directory
```

Only `/health` and bounded `/t27` requests are served. Stop this own container
with Ctrl-C. Do not expose unencrypted HTTP beyond the trusted local frontend.
An ephemeral HTTPS tunnel proves ingress only, not native UDP or NAT traversal.
An operator endpoint sees public identities, timing, candidate addresses and
ciphertext. Private keys and decrypted messages stay at the endpoints.
