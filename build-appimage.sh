#!/usr/bin/env bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APPDIR="${REPO_ROOT}/target/NetLens.AppDir"
OUTPUT_DIR="${REPO_ROOT}/dist"
APPIMAGE_TOOL="${HOME}/.local/bin/appimagetool"

echo "==========================================================="
echo "📦 NetLens AppImage Generator"
echo "==========================================================="

# Verify appimagetool
if [ ! -f "${APPIMAGE_TOOL}" ]; then
    echo "Downloading appimagetool to ${APPIMAGE_TOOL}..."
    mkdir -p "$(dirname "${APPIMAGE_TOOL}")"
    curl -Lo "${APPIMAGE_TOOL}" https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
    chmod +x "${APPIMAGE_TOOL}"
fi

echo "==> 1. Verifying compilation in optimized Release mode..."
cargo build --release --manifest-path "${REPO_ROOT}/Cargo.toml"

echo "==> 2. Preparing AppDir structure..."
rm -rf "${APPDIR}"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/512x512/apps"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${OUTPUT_DIR}"

# Executable binary
cp "${REPO_ROOT}/target/release/netlens" "${APPDIR}/usr/bin/netlens"
chmod +x "${APPDIR}/usr/bin/netlens"

# Icons and Desktop Entry
cp "${REPO_ROOT}/assets/netlens.desktop" "${APPDIR}/netlens.desktop"
cp "${REPO_ROOT}/assets/netlens.desktop" "${APPDIR}/usr/share/applications/netlens.desktop"
cp "${REPO_ROOT}/assets/netlens.png" "${APPDIR}/netlens.png"
cp "${REPO_ROOT}/assets/netlens.png" "${APPDIR}/.DirIcon"
cp "${REPO_ROOT}/assets/netlens.png" "${APPDIR}/usr/share/icons/hicolor/512x512/apps/netlens.png"
cp "${REPO_ROOT}/assets/netlens.svg" "${APPDIR}/netlens.svg"

# Standard AppRun bootstrap script
cat << 'EOF' > "${APPDIR}/AppRun"
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"

# Preserve or export persistent AppImage executable path
if [ -z "${APPIMAGE}" ]; then
    if [ -n "${TARGET_APPIMAGE}" ]; then
        export APPIMAGE="${TARGET_APPIMAGE}"
    elif [ -n "${ARGV0}" ]; then
        export APPIMAGE="$(readlink -f "${ARGV0}")"
    fi
fi

exec "${HERE}/usr/bin/netlens" "$@"
EOF
chmod +x "${APPDIR}/AppRun"

echo "==> 3. Packaging standalone AppImage with zstd compression..."
export ARCH=x86_64
rm -f "${OUTPUT_DIR}/NetLens-x86_64.AppImage"
"${APPIMAGE_TOOL}" --appimage-extract-and-run "${APPDIR}" "${OUTPUT_DIR}/NetLens-x86_64.AppImage"

echo ""
echo "==========================================================="
echo "🎉 AppImage generated successfully!"
ls -lh "${OUTPUT_DIR}/NetLens-x86_64.AppImage"
echo "==========================================================="
