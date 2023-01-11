#!/bin/sh
# Sometimes, by the time we get around to packaging cargo, the crates.io
# ecosystem has moved on from the versions that cargo-vendor *would have* used
# when that version of cargo was released. This places a lot of maintenance
# burden on us, since it requires us to keep debcargo-conf always up-to-date.
#
# This script allows us to force cargo-vendor to use old versions of dependency
# crates, that better match our debcargo-conf patches, as well as the versions
# of crates used when a particular version of cargo was actually released.
#
# Example:
# cargo update -p filetime --precise 0.2.12
#

# Drop the "vendored" feature since we patch it out of debcargo
sed -i /vendored/d Cargo.toml

# avoid pulling in windows-sys for now
cargo update -p schannel --precise 0.1.19
