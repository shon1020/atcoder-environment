#!/bin/bash
# =============================================================================
# setup.sh - atcoder-environment セットアップスクリプト
#
# 【概要】
#   新しいマシンでこのリポジトリをクローンした後に実行する。
#   cargo-compete のデータディレクトリをリポジトリ内で管理するための
#   symlink を作成し、.env の内容から認証用 cookies.jsonl を生成する。
#
# 【使い方】
#   1. .env に REVEL_SESSION の値を記入する（取得方法は下記参照）
#   2. ./setup.sh を実行する
#
# 【REVEL_SESSION の取得方法】
#   1. ブラウザで https://atcoder.jp にログイン
#   2. DevTools (F12) → Application → Cookies → https://atcoder.jp
#   3. REVEL_SESSION の値をコピーして .env に貼り付ける
#
# 【ファイル構成】
#   .env                        ... REVEL_SESSION を記載（gitignore済み・要作成）
#   .cargo-compete/             ... cargo-compete のデータディレクトリ（リポジトリ管理）
#   .cargo-compete/cookies.jsonl ... このスクリプトが .env から自動生成（gitignore済み）
#   ~/Library/Application Support/cargo-compete → .cargo-compete への symlink
# =============================================================================
set -e

# --- パス定義 ----------------------------------------------------------------

# このスクリプトがあるディレクトリ（リポジトリルート）
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# リポジトリ内の cargo-compete データディレクトリ
CARGO_COMPETE_DIR="$REPO_DIR/.cargo-compete"

# cargo-compete がデフォルトで参照するシステム側のパス（macOS）
SYSTEM_DIR="$HOME/Library/Application Support/cargo-compete"

# 認証情報ファイル
ENV_FILE="$REPO_DIR/.env"
COOKIES_FILE="$CARGO_COMPETE_DIR/cookies.jsonl"

# --- symlink のセットアップ --------------------------------------------------
# cargo-compete がシステムパスを参照したとき、リポジトリ内のディレクトリを
# 見るように symlink を張る。

mkdir -p "$CARGO_COMPETE_DIR"

if [ -L "$SYSTEM_DIR" ]; then
    # すでに symlink が存在する場合はスキップ
    echo "symlink already exists: $SYSTEM_DIR"
elif [ -d "$SYSTEM_DIR" ]; then
    # 既存のディレクトリがある場合はバックアップしてから symlink を作成
    echo "existing directory found, moving to ${CARGO_COMPETE_DIR}.bak ..."
    mv "$SYSTEM_DIR" "${CARGO_COMPETE_DIR}.bak"
    ln -s "$CARGO_COMPETE_DIR" "$SYSTEM_DIR"
    echo "symlink created"
else
    # 親ディレクトリがない場合は作成してから symlink を作成
    mkdir -p "$(dirname "$SYSTEM_DIR")"
    ln -s "$CARGO_COMPETE_DIR" "$SYSTEM_DIR"
    echo "symlink created"
fi

# --- .env から cookies.jsonl を生成 -----------------------------------------
# cargo-compete は cookies.jsonl を読んで AtCoder の認証を行う。
# 秘密情報を直接 cookies.jsonl に書かないよう、.env を正とし
# ここで変換して生成する。

if [ ! -f "$ENV_FILE" ]; then
    echo ""
    echo "エラー: .env が見つかりません。"
    echo "REVEL_SESSION=<値> を記載した .env をリポジトリルートに作成してください。"
    exit 1
fi

# .env を読み込む（REVEL_SESSION が環境変数にセットされる）
source "$ENV_FILE"

if [ -z "$REVEL_SESSION" ]; then
    echo ""
    echo "エラー: .env に REVEL_SESSION が設定されていません。"
    echo "ブラウザの DevTools から値を取得して .env に記入してください。"
    exit 1
fi

# cookies.jsonl を生成（cookie_store クレートが要求するフォーマット）
# domain は {"HostOnly": "..."} の enum 形式が必要
cat > "$COOKIES_FILE" <<EOF
{"raw_cookie":"REVEL_SESSION=${REVEL_SESSION}; HttpOnly; Secure; SameSite=Lax; Path=/","path":["/",true],"domain":{"HostOnly":"atcoder.jp"},"expires":"SessionEnd"}
EOF

echo "cookies.jsonl を生成しました"
echo ""
echo "セットアップ完了"
