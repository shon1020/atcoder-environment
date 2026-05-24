# atcoder-environment
atcoderをrustで解くためのリポジトリ

ユーザー名：shonNMIXX

[https://atcoder.jp/users/shonNMIXX](https://atcoder.jp/users/shonNMIXX)

## atcoderにログインする
`.env`ファイルを作成する。webサイトのデベロッパーツールからセッションIDを取得後に
`REVEAL_SESSION`の値に追加する
```bash
REVEAL_SESSION="<session_id>"
```
その後に`setup.sh`を実行する

```
./setup.sh
```

## コンテスト用のファイルを取得
```bash
./compete-new.sh abc<hoge>
```

## テストコマンド
各コンテストのディレクトリ内で下記のコマンドを実行する
```bash
cargo compete test <problem>
```

## 提出コマンド
各コンテストのディレクトリ内で下記のコマンドを実行する
```bash
cargp compete submit <probelm>
```
