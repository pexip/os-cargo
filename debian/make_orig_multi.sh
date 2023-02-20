#!/bin/sh
set -e
echo "This needs the following packages:"
echo "  python3-pytoml devscripts cargo"
echo ""

TMPDIR=`mktemp -d`
echo "Using '${TMPDIR}'..."
cat > "${TMPDIR}/Makefile" <<'EOF'
include /usr/share/dpkg/pkg-info.mk
all:
	@echo $(DEB_VERSION_UPSTREAM)
EOF
SRCDIR="$PWD"

if [ -z "$1" ]
  then
    USCAN_ARGS="";
    CARGO_VER=$(make -f "${TMPDIR}/Makefile");
  else
    USCAN_ARGS="--download-version $1";
    CARGO_VER="$1";
fi;

BOOTSTRAP_PY=$(find "${PWD}" -name bootstrap.py -type f)
VENDOR_SUS_WHITELIST=$(find "${PWD}/debian" -name vendor-tarball-unsuspicious.txt -type f)

# Download cargo tarball
uscan --rename ${USCAN_ARGS} --force-download --destdir "${TMPDIR}/"

# Extract cargo source
cd "${TMPDIR}"
mkdir cargo
tar -xaf "${TMPDIR}/cargo_${CARGO_VER}.orig.tar.gz" -C cargo --strip-components=1
cd cargo

# special patch for CVE fix - we want to vendor the updated/fixed dependencies!
echo "Applying CVE-2022-46176 patches";
for p in "${SRCDIR}/debian/patches/cve/"*.patch; do
  echo "$(basename "$p")"
  patch -p1 < "$p"
  echo "$p" >> .cve-patches
done

# Download build-deps via cargo-vendor
export GIT_AUTHOR_NAME="deb-build"
export GIT_AUTHOR_EMAIL="<>"
export GIT_COMMITTER_NAME="${GIT_AUTHOR_NAME}"
export GIT_COMMITTER_EMAIL="${GIT_AUTHOR_EMAIL}"

SRCDIR="$SRCDIR" \
CARGO_PRE_VENDOR="$SRCDIR/debian/make_orig_multi-pre-vendor.sh" \
"$SRCDIR/debian/scripts/debian-cargo-vendor"

cp -R vendor vendor-scan

( cd vendor-scan
"$SRCDIR/debian/scripts/audit-vendor-source" \
  "$VENDOR_SUS_WHITELIST" \
  "the 'excludes' key of the relevant debcargo.toml in debcargo-conf.git" \
  -m text/x-script.python
)

rm -rf vendor-scan

# special patch for CVE fix - unapply to keep orig.tar.gz pristine
echo "Unapplying CVE-2022-46176 patches";
tac .cve-patches | while read p; do
  echo "$(basename "$p")"
  patch -Rp1 < "$p"
done
rm .cve-patches

# Pack it up, reproducibly
tar --sort=name \
    --use-compress-program='gzip -9n' \
    --mtime="./Cargo.lock" \
    --owner=root --group=root \
    -cf "${TMPDIR}/cargo_${CARGO_VER}.orig-vendor.tar.gz" vendor

# All is good, we are done!
echo "Your files are available at:"
echo "${TMPDIR}/cargo_${CARGO_VER}.orig.tar.gz \\"
echo "${TMPDIR}/cargo_${CARGO_VER}.orig-vendor.tar.gz"
echo ""
echo "Unpacked cargo sources are available under:"
echo "${TMPDIR}/cargo/"
