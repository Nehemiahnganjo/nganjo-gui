#!/usr/bin/env bash
set -e

echo "╔══════════════════════════════════════╗"
echo "║        NeoDesktop Installer          ║"
echo "╚══════════════════════════════════════╝"
echo ""

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "[!] Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "[+] Rust: $(rustc --version)"

# Check for system deps (Arch-based)
if command -v pacman &> /dev/null; then
    echo "[+] Detected Arch-based system"
    MISSING=()
    for pkg in libxkbcommon wayland mesa; do
        if ! pacman -Qs "$pkg" &>/dev/null; then
            MISSING+=("$pkg")
        fi
    done
    if [ ${#MISSING[@]} -gt 0 ]; then
        echo "[!] Installing missing deps: ${MISSING[*]}"
        sudo pacman -S --noconfirm "${MISSING[@]}" 2>/dev/null || true
    fi
fi

# Build
echo ""
echo "[+] Building NeoDesktop (release)..."
cargo build --release

# Install binary
echo "[+] Installing binary to /usr/local/bin/neodesktop"
sudo install -Dm755 target/release/neodesktop /usr/local/bin/neodesktop

# Install .desktop entry
echo "[+] Installing .desktop entry..."
sudo install -Dm644 neodesktop.desktop /usr/share/xsessions/neodesktop.desktop

# Optional: set as default session
echo ""
echo "╔══════════════════════════════════════╗"
echo "║   NeoDesktop installed successfully! ║"
echo "╠══════════════════════════════════════╣"
echo "║  To use NeoDesktop:                  ║"
echo "║  1. Log out of your current session  ║"
echo "║  2. Select 'NeoDesktop' at login     ║"
echo "║  3. Or run: neodesktop               ║"
echo "╚══════════════════════════════════════╝"
