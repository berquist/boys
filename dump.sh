#!/bin/bash

# dump.sh: Take a look at the generated assembly before an after using a
# stdlib function that might hint to the compiler to do FMA.

set -eu -o pipefail

git checkout 9edf42871a13cd8a1bc8adfd3bbde29c33753878
cargo asm --lib boys::exact::boys >before.s
git checkout main
# Otherwise we don't see the code
SED=sed
if command -v gsed >&/dev/null; then
	SED=gsed
fi
${SED} -i 's/#\[inline\]/#[inline(never)]/g' src/micb25/mod.rs src/exact/mod.rs
cargo asm --lib boys::exact::boys >after.s
git checkout -- .
