<p align="center"><img src="installer/linux/lj-hub.svg" width="112" alt=""></p>

<h1 align="center">LJ Hub</h1>

<p align="center"><b>Logitech デバイスのローカルなプロファイルと一覧。DPI とレポートレートの調整は実験的機能です。</b></p>

<p align="center"><img src="https://img.shields.io/badge/%F0%9F%9B%A0%EF%B8%8F%20System%20%26%20hardware-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f6fde?style=flat-square" alt="システムとハードウェア"> <a href="https://github.com/jeremdevgit/lj-hub/releases"><img src="https://img.shields.io/github/v/release/jeremdevgit/lj-hub?include_prereleases&style=flat-square&label=version" alt="version"></a> <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-555?style=flat-square" alt="GPL-3.0-or-later"></a> <a href="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml"><img src="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml/badge.svg" alt="CI"></a></p>

<p align="center"><a href="README.md">English</a> · <a href="README.fr.md">Français</a> · <a href="README.es.md">Español</a> · <a href="README.de.md">Deutsch</a> · <a href="README.it.md">Italiano</a> · <a href="README.pt.md">Português</a> · <a href="README.zh.md">简体中文</a> · <b>日本語</b></p>

<p align="center"><a href="https://github.com/jeremdevgit/lj-hub/releases"><b>⬇ ダウンロード</b></a></p>

![LJ Hub](docs/screenshots/home-en.png)

LJ Hub は Logitech ゲーミングデバイスのための、軽量でローカル完結の管理ツールです。**初期アルファ版で、まだ G HUB の代わりにはなりません。** Logitech とは関係のない独立したプロジェクトです。

<p><img src="docs/screenshots/device-en.png" alt="デバイスページ" width="49%"> <img src="docs/screenshots/settings-en.png" alt="設定" width="49%"></p>

## 概要

| | |
|---|---|
| **カテゴリー** | 🛠️ システムとハードウェア |
| **対応環境** | Windows · macOS · Linux |
| **表示言語** | 8 言語（英・仏・西・独・伊・葡・中・日） |
| **価格** | 無料・オープンソース・広告なし・テレメトリーなし |
| **ステータス** | アルファ版 |
| **ライセンス** | [GPL-3.0-or-later](LICENSE) |

## 現在できること

| 機能 | Windows | macOS | Linux |
|---|:-:|:-:|:-:|
| Logitech の USB HID デバイスを検出 | ✅ | ✅ | ✅ |
| ローカルプロファイルの作成・編集・複製・バックアップ（JSON） | ✅ | ✅ | ✅ |
| カテゴリー別プレビューと機能状態を表示するダッシュボード | ✅ | ✅ | ✅ |
| Logitech の 116 個の ID を検索（固定版 libratbag カタログ） | ✅ | ✅ | ✅ |
| システムの USB/PnP 一覧（カメラ、ハンコンなど） | ✅ | ✅ | ✅ |
| DPI とレポートレートの設定 | 🧪 HID++ 2.0 の有線マウス | — | 🧪 `ratbagd` 経由 |
| LED 0 の固定色 | — | — | 🧪 任意、`ratbagd` 経由 |

🧪 = 実験的機能。実機ではまだ検証していません。

ライティングを [rJb](https://github.com/jeremdevgit/rjb) に任せられるよう、RGB は**既定でオフ**です。マクロ、キー割り当て、LIGHTSYNC エフェクト、オーディオ、Web カメラ、ハンコン、ファームウェア、ゲーム検出、プロファイルの自動切り替えは未実装です。一覧に表示されても設定できるとは限りません。

## インストール

[リリースページ](https://github.com/jeremdevgit/lj-hub/releases)から最新のプレリリースをダウンロードします：Windows はユーザー単位の `.exe` インストーラー、macOS は Apple Silicon 用 `.dmg`、Linux は Ubuntu/Debian x64 用 `.deb`。Windows と macOS のパッケージは信頼された発行元の証明書で署名されていません。Linux でハードウェアを制御するには `ratbagd`、`ratbagctl` と libratbag 対応デバイスが必要です。

## 安全性とプライバシー

- **ネットワーク通信なし、アカウント不要。** プロファイルはシステムのユーザー設定フォルダーに保存されます。
- ハードウェアへの書き込みは任意で、デバイスが公開する値と照合し、書き込み後に読み戻して確認します。テスト済みのデバイスから順に有効化します。

## ロードマップ

1. 各 OS の実機で HID++ の機能を検証する。
2. DPI、ポーリングレート、RGB、オンボードプロファイルのテスト済み機能表。
3. ボタン割り当てと制限付きマクロエンジン。
4. プロトコルが公開されているヘッドセット、マイク、Web カメラ、ハンコンへの対応。
5. ゲームごとのプロファイル切り替え。

## ソースからビルド

```sh
# Rust stable。Linux ではさらに：libudev-dev libx11-dev libxi-dev libxcursor-dev
# libxrandr-dev libxkbcommon-dev libwayland-dev pkg-config
cargo run --locked --release
```

デバイスカタログは [libratbag のデバイスファイル](https://github.com/libratbag/libratbag/tree/master/data/devices)から、`data/libratbag-logitech.json` に記録したコミットで生成しています。更新：`python3 tools/refresh_libratbag_catalog.py /path/to/libratbag`。変更履歴：[CHANGELOG.md](CHANGELOG.md)。

## ライセンス

GPL-3.0-or-later — [LICENSE](LICENSE) を参照。Logitech、G HUB、LIGHTSYNC は各所有者の商標です。

---

脆弱性を見つけたら、公開 Issue ではなく非公開で報告してください（[SECURITY.md](SECURITY.md) を参照）。

## jeremdevgit のその他のフリーソフトウェア

すべて無料・オープンソース・広告なし。

**🛠️ システムとハードウェア** — [Boîte à outils](https://github.com/jeremdevgit/boite-a-outils) · [rJb](https://github.com/jeremdevgit/rjb) · [Decklight](https://github.com/jeremdevgit/decklight)

**📒 生産性とデータ** — [Carnet de notes](https://github.com/jeremdevgit/carnet-rust) · [JMeca](https://github.com/jeremdevgit/jmeca) · [Suite Libre](https://jeremdevgit.github.io/suite-libre/)

**🎮 音楽とゲーム** — [Pomme](https://github.com/jeremdevgit/pomme) · [Ludothèque](https://github.com/jeremdevgit/ludotheque) · [AutoDex](https://github.com/jeremdevgit/autodex) · [Batailles de France](https://github.com/jeremdevgit/batailles-de-france)
