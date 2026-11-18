#!/usr/bin/env bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PKG_DIR="${REPO_ROOT}/packaging/archlinux"
DIST_REPO="${REPO_ROOT}/dist/repo/x86_64"
EXTERNAL_REPO="${REPO_ROOT}/../netlens-repo/x86_64"
RELEASE_BIN="${REPO_ROOT}/target/release/netlens"

echo "============================================================"
echo "📦 Pacman / Shelly Repository Generator for NetLens"
echo "============================================================"

# 1. Ensure compilation in Release mode
if [ ! -f "${RELEASE_BIN}" ]; then
    echo "==> 1. Building NetLens in optimized Release mode..."
    cargo build --release --manifest-path "${REPO_ROOT}/Cargo.toml"
else
    echo "==> 1. Release binary found at ${RELEASE_BIN}"
fi

# 2. Prepare packaging staging directory
BUILD_TMP=$(mktemp -d)
trap 'rm -rf "${BUILD_TMP}"' EXIT

echo "==> 2. Preparing source files for makepkg..."
cp "${PKG_DIR}/PKGBUILD" "${BUILD_TMP}/"
cp "${RELEASE_BIN}" "${BUILD_TMP}/netlens"
cp "${REPO_ROOT}/assets/netlens.desktop" "${BUILD_TMP}/"
cp "${REPO_ROOT}/assets/netlens.png" "${BUILD_TMP}/"
cp "${REPO_ROOT}/assets/netlens.svg" "${BUILD_TMP}/"

# 3. Build package with makepkg
echo "==> 3. Building .pkg.tar.zst package..."
(
    cd "${BUILD_TMP}"
    makepkg -f --nodeps
)

PKG_FILE=$(find "${BUILD_TMP}" -maxdepth 1 -name "*.pkg.tar.zst" -type f | head -n 1)
if [ -z "${PKG_FILE}" ]; then
    echo "❌ Error: Could not generate .pkg.tar.zst package"
    exit 1
fi
PKG_NAME=$(basename "${PKG_FILE}")
echo "✅ Package created: ${PKG_NAME}"

# 4. Create repository layout
echo "==> 4. Organizing Pacman repository at ${DIST_REPO}..."
mkdir -p "${DIST_REPO}"
rm -f "${DIST_REPO}"/netlens-*.pkg.tar.zst
cp "${PKG_FILE}" "${DIST_REPO}/"

# 5. Generate / Update package database with shelly or repo-add
echo "==> 5. Updating package database with shelly repo-db / repo-add..."
(
    cd "${DIST_REPO}"
    if command -v shelly >/dev/null 2>&1; then
        shelly repo-db add -R netlens.db.tar.zst "${PKG_NAME}"
    else
        repo-add -R netlens.db.tar.zst "${PKG_NAME}"
    fi

    # Create hard copies for static servers like GitHub Pages
    rm -f netlens.db netlens.files
    cp netlens.db.tar.zst netlens.db
    cp netlens.files.tar.zst netlens.files
    rm -f *.lock *.old
)

# 6. Synchronize with external repository if present
if [ -d "${REPO_ROOT}/../netlens-repo" ]; then
    echo "==> 6. Synchronizing with GitHub Pages repo at ${EXTERNAL_REPO}..."
    mkdir -p "${EXTERNAL_REPO}"
    rm -f "${EXTERNAL_REPO}"/netlens-*.pkg.tar.zst
    cp -r "${DIST_REPO}"/* "${EXTERNAL_REPO}/"
fi

echo ""
echo "============================================================"
echo "🎉 Pacman repository successfully generated at:"
echo "   ${DIST_REPO}"
if [ -d "${REPO_ROOT}/../netlens-repo" ]; then
    echo "   ${EXTERNAL_REPO}"
fi
echo "============================================================"
ls -lh "${DIST_REPO}"
