#!/usr/bin/env bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PKG_DIR="${REPO_ROOT}/packaging/archlinux"
DIST_ROOT="${REPO_ROOT}/dist/repo"
DIST_REPO="${DIST_ROOT}/x86_64"
EXTERNAL_REPO_ROOT="${REPO_ROOT}/../netlens-repo"
EXTERNAL_REPO="${EXTERNAL_REPO_ROOT}/x86_64"
RELEASE_BIN="${REPO_ROOT}/target/release/netlens"

echo "============================================================"
echo "📦 Pacman / Shelly Repository Generator for NetLens"
echo "============================================================"

# 0. Single Source of Truth: Extract version from Cargo.toml
APP_VERSION=$(grep -m1 '^version' "${REPO_ROOT}/Cargo.toml" | cut -d '"' -f2)
TODAY=$(date +%Y-%m-%d)
echo "==> 0. Detected version from Cargo.toml: ${APP_VERSION}"

# Automatically synchronize PKGBUILD pkgver
sed -i -E "s/^pkgver=.*/pkgver=${APP_VERSION}/" "${PKG_DIR}/PKGBUILD"
echo "✅ Synced PKGBUILD: pkgver=${APP_VERSION}"

# 1. Compile in Release mode
echo "==> 1. Building NetLens in optimized Release mode..."
cargo build --release --manifest-path "${REPO_ROOT}/Cargo.toml"

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

# 6. Generate synchronized version.json
echo "==> 6. Generating synchronized version.json..."
VERSION_JSON=$(cat << EOF
{
  "version": "${APP_VERSION}",
  "pkgrel": 1,
  "release_date": "${TODAY}",
  "changelog": "NetLens ${APP_VERSION}: Actualización de versión",
  "url": "https://omarmurcia.github.io/netlens-repo"
}
EOF
)

mkdir -p "${DIST_ROOT}"
echo "${VERSION_JSON}" > "${DIST_ROOT}/version.json"

# 7. Synchronize with external repository if present
if [ -d "${EXTERNAL_REPO_ROOT}" ]; then
    echo "==> 7. Synchronizing with GitHub Pages repo at ${EXTERNAL_REPO_ROOT}..."
    mkdir -p "${EXTERNAL_REPO}"
    rm -f "${EXTERNAL_REPO}"/netlens-*.pkg.tar.zst
    cp -r "${DIST_REPO}"/* "${EXTERNAL_REPO}/"
    echo "${VERSION_JSON}" > "${EXTERNAL_REPO_ROOT}/version.json"
fi

echo ""
echo "============================================================"
echo "🎉 Pacman repository & version.json successfully generated at:"
echo "   ${DIST_REPO}"
echo "   Version: ${APP_VERSION} (Date: ${TODAY})"
if [ -d "${EXTERNAL_REPO_ROOT}" ]; then
    echo "   ${EXTERNAL_REPO_ROOT}/version.json"
fi
echo "============================================================"
ls -lh "${DIST_REPO}"
