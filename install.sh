#!/usr/bin/env bash
set -euo pipefail

# Loom — Installer for macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/kevin-espineira/loom-oc/main/install.sh | bash

REPO="kevin-espineira/loom-oc"
APP_NAME="Loom"
INSTALL_DIR="/Applications"

# --- Color helpers ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()  { echo -e "${CYAN}ℹ  $*${NC}"; }
ok()    { echo -e "${GREEN}✓  $*${NC}"; }
warn()  { echo -e "${YELLOW}⚠  $*${NC}"; }
error() { echo -e "${RED}✗  $*${NC}"; }

# --- Pre-flight checks ---
if [[ "$(uname)" != "Darwin" ]]; then
  error "This installer is for macOS only."
  exit 1
fi

# --- Detect architecture ---
ARCH=$(uname -m)
case "$ARCH" in
  arm64)  DMARCH="aarch64" ;;
  x86_64) DMARCH="x86_64" ;;
  *)
    error "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

info "Detected architecture: $ARCH"

# --- Fetch latest release ---
info "Fetching latest release from GitHub..."
LATEST_URL="https://api.github.com/repos/$REPO/releases/latest"
LATEST_JSON=$(curl -fsSL "$LATEST_URL" 2>/dev/null) || {
  error "Failed to fetch latest release. Check your internet connection."
  exit 1
}

LATEST_VERSION=$(echo "$LATEST_JSON" | grep -oP '"tag_name":\s*"\K[^"]+') || {
  error "Could not parse version from GitHub response."
  exit 1
}

info "Latest version: $LATEST_VERSION"

# --- Find the correct .dmg asset ---
DMG_URL=$(echo "$LATEST_JSON" | grep -oP '"browser_download_url":\s*"\K[^"]+' | grep -E "loom_${DMARCH}|${APP_NAME}.*${DMARCH}|${DMARCH}.*dmg" | head -1) || true

# Fallback: try any .dmg if arch-specific not found
if [[ -z "$DMG_URL" ]]; then
  DMG_URL=$(echo "$LATEST_JSON" | grep -oP '"browser_download_url":\s*"\K[^"]+' | grep '\.dmg$' | head -1) || true
fi

if [[ -z "$DMG_URL" ]]; then
  error "Could not find a .dmg asset for $ARCH in release v$LATEST_VERSION."
  error "Available assets:"
  echo "$LATEST_JSON" | grep -oP '"browser_download_url":\s*"\K[^"]+' | grep '\.dmg$' || echo "  (none)"
  exit 1
fi

info "Downloading $APP_NAME v$LATEST_VERSION..."

# --- Download .dmg ---
TMPDIR_BASE=$(mktemp -d /tmp/loom-install.XXXXXX)
trap 'rm -rf "$TMPDIR_BASE"' EXIT INT TERM

DMG_PATH="$TMPDIR_BASE/${APP_NAME}_${LATEST_VERSION}.dmg"

curl -fSL -o "$DMG_PATH" "$DMG_URL" || {
  error "Download failed. Check your internet connection."
  exit 1
}

DMG_SIZE=$(stat -f%z "$DMG_PATH" 2>/dev/null || stat -c%s "$DMG_PATH" 2>/dev/null)
DMG_SIZE_MB=$((DMG_SIZE / 1024 / 1024))
info "Downloaded ${DMG_SIZE_MB}MB"

# --- Mount .dmg ---
info "Mounting installer..."
MOUNT_NAME="${APP_NAME}_${LATEST_VERSION}"
MOUNT_PATH="/Volumes/$MOUNT_NAME"

hdiutil attach "$DMG_PATH" -nobrowse -quiet || {
  error "Failed to mount .dmg file."
  exit 1
}

# --- Find .app inside .dmg ---
APP_BUNDLE=$(find "$MOUNT_PATH" -maxdepth 1 -name "*.app" -type d 2>/dev/null | head -1) || true

if [[ -z "$APP_BUNDLE" ]]; then
  error "No .app bundle found in the installer."
  hdiutil detach "$MOUNT_PATH" -quiet 2>/dev/null || true
  exit 1
fi

# --- Check if already installed ---
EXISTING_APP="$INSTALL_DIR/$APP_NAME.app"
if [[ -d "$EXISTING_APP" ]]; then
  warn "$APP_NAME is already installed in $INSTALL_DIR"
  
  EXISTING_VERSION=$(defaults read "$EXISTING_APP/Contents/Info.plist" CFBundleShortVersionString 2>/dev/null || echo "unknown")
  
  if [[ "$EXISTING_VERSION" == "$LATEST_VERSION" ]]; then
    ok "Version $LATEST_VERSION is already installed. Nothing to do."
    hdiutil detach "$MOUNT_PATH" -quiet 2>/dev/null || true
    exit 0
  fi
  
  info "Backing up existing version..."
  cp -R "$EXISTING_APP" "$TMPDIR_BASE/${APP_NAME}_old.app"
fi

# --- Copy .app to /Applications ---
info "Installing to $INSTALL_DIR..."

cp -R "$APP_BUNDLE" "$INSTALL_DIR/" || {
  error "Failed to install. Check permissions for $INSTALL_DIR."
  exit 1
}

ok "Installed $APP_NAME v$LATEST_VERSION to $INSTALL_DIR"

# --- Unmount .dmg ---
hdiutil detach "$MOUNT_PATH" -quiet 2>/dev/null || true

# --- Cleanup old version ---
if [[ -d "$TMPDIR_BASE/${APP_NAME}_old.app" ]]; then
  info "Removing old version..."
  rm -rf "$TMPDIR_BASE/${APP_NAME}_old.app"
fi

# --- Open app ---
if [[ "${1:-}" == "--open" ]]; then
  info "Opening $APP_NAME..."
  open -a "$APP_NAME"
fi

ok "Done! You can launch $APP_NAME from $INSTALL_DIR or Spotlight."
info "Release notes: https://github.com/$REPO/releases/tag/v$LATEST_VERSION"
