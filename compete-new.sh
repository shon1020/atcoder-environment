#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CARGO_TOML="$SCRIPT_DIR/Cargo.toml"
TEMPLATE="$SCRIPT_DIR/snippets/snippet.rs"

cargo compete new "$@"

CONTEST="${@: -1}"

# 生成された各 bin ファイルを snippets/snippet.rs の内容で上書き
BIN_DIR="$SCRIPT_DIR/src/${CONTEST}/src/bin"
if [ -f "$TEMPLATE" ] && [ -d "$BIN_DIR" ]; then
    for binfile in "$BIN_DIR"/*.rs; do
        [ -f "$binfile" ] || continue
        cp "$TEMPLATE" "$binfile"
    done
    echo "[template] $BIN_DIR/*.rs を snippets/snippet.rs で初期化しました"
fi

if grep -qF "\"src/${CONTEST}\"" "$CARGO_TOML"; then
    echo "[workspace] src/${CONTEST} は既に Cargo.toml に登録済みです"
    exit 0
fi

# members の閉じ ] の直前に新エントリを挿入（最初の ] のみ対象）
awk -v entry="    \"src/${CONTEST}\"," '
    /^\]$/ && !done { print entry; done=1 }
    { print }
' "$CARGO_TOML" > "${CARGO_TOML}.tmp" && mv "${CARGO_TOML}.tmp" "$CARGO_TOML"

echo "[workspace] src/${CONTEST} を Cargo.toml に追加しました"
