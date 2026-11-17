#!/usr/bin/env bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APPDIR="${REPO_ROOT}/target/NetLens.AppDir"
OUTPUT_DIR="${REPO_ROOT}/dist"
APPIMAGE_TOOL="${HOME}/.local/bin/appimagetool"

echo "==========================================================="
echo "📦 Generador de AppImage para NetLens"
echo "==========================================================="

# Verificar appimagetool
if [ ! -f "${APPIMAGE_TOOL}" ]; then
    echo "Descargando appimagetool en ${APPIMAGE_TOOL}..."
    mkdir -p "$(dirname "${APPIMAGE_TOOL}")"
    curl -Lo "${APPIMAGE_TOOL}" https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
    chmod +x "${APPIMAGE_TOOL}"
fi

echo "==> 1. Verificando compilación en modo Release optimizado..."
cargo build --release --manifest-path "${REPO_ROOT}/Cargo.toml"

echo "==> 2. Preparando estructura AppDir..."
rm -rf "${APPDIR}"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/512x512/apps"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${OUTPUT_DIR}"

# Binario ejecutable
cp "${REPO_ROOT}/target/release/netlens" "${APPDIR}/usr/bin/netlens"
chmod +x "${APPDIR}/usr/bin/netlens"

# Iconos y Desktop Entry
cp "${REPO_ROOT}/assets/netlens.desktop" "${APPDIR}/netlens.desktop"
cp "${REPO_ROOT}/assets/netlens.desktop" "${APPDIR}/usr/share/applications/netlens.desktop"
cp "${REPO_ROOT}/assets/netlens.png" "${APPDIR}/netlens.png"
cp "${REPO_ROOT}/assets/netlens.png" "${APPDIR}/.DirIcon"
cp "${REPO_ROOT}/assets/netlens.png" "${APPDIR}/usr/share/icons/hicolor/512x512/apps/netlens.png"
cp "${REPO_ROOT}/assets/netlens.svg" "${APPDIR}/netlens.svg"

# Script estándar de arranque AppRun
cat << 'EOF' > "${APPDIR}/AppRun"
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"

# Preservar o exportar la ruta permanente del archivo AppImage
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

echo "==> 3. Empaquetando AppImage independiente con compresión zstd..."
export ARCH=x86_64
rm -f "${OUTPUT_DIR}/NetLens-x86_64.AppImage"
"${APPIMAGE_TOOL}" --appimage-extract-and-run "${APPDIR}" "${OUTPUT_DIR}/NetLens-x86_64.AppImage"

echo ""
echo "==========================================================="
echo "🎉 ¡AppImage generado con éxito!"
ls -lh "${OUTPUT_DIR}/NetLens-x86_64.AppImage"
echo "==========================================================="
