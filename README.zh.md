<p align="center"><img src="installer/linux/lj-hub.svg" width="112" alt=""></p>

<h1 align="center">LJ Hub</h1>

<p align="center"><b>罗技设备的本地配置文件与设备清单，并提供实验性的 DPI 和回报率调节。</b></p>

<p align="center"><img src="https://img.shields.io/badge/%F0%9F%9B%A0%EF%B8%8F%20System%20%26%20hardware-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f6fde?style=flat-square" alt="系统与硬件"> <a href="https://github.com/jeremdevgit/lj-hub/releases"><img src="https://img.shields.io/github/v/release/jeremdevgit/lj-hub?include_prereleases&style=flat-square&label=version" alt="version"></a> <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-555?style=flat-square" alt="GPL-3.0-or-later"></a> <a href="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml"><img src="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml/badge.svg" alt="CI"></a></p>

<p align="center"><a href="README.md">English</a> · <a href="README.fr.md">Français</a> · <a href="README.es.md">Español</a> · <a href="README.de.md">Deutsch</a> · <a href="README.it.md">Italiano</a> · <a href="README.pt.md">Português</a> · <b>简体中文</b> · <a href="README.ja.md">日本語</a></p>

<p align="center"><a href="https://github.com/jeremdevgit/lj-hub/releases"><b>⬇ 下载</b></a></p>

![LJ Hub](docs/screenshots/home-en.png)

LJ Hub 是罗技游戏设备的轻量本地管理工具。**目前处于早期 Alpha 阶段，还不能替代 G HUB**；这是一个独立项目，与罗技无关。

<p><img src="docs/screenshots/device-en.png" alt="设备页面" width="49%"> <img src="docs/screenshots/settings-en.png" alt="设置" width="49%"></p>

## 概览

| | |
|---|---|
| **类别** | 🛠️ 系统与硬件 |
| **平台** | Windows · macOS · Linux |
| **界面语言** | 8 种语言（英、法、西、德、意、葡、中、日） |
| **价格** | 免费、开源、无广告、无遥测 |
| **状态** | 早期 Alpha |
| **许可证** | [GPL-3.0-or-later](LICENSE) |

## 目前可用的功能

| 功能 | Windows | macOS | Linux |
|---|:-:|:-:|:-:|
| 发现罗技 USB HID 设备 | ✅ | ✅ | ✅ |
| 创建、编辑、复制和备份本地配置文件（JSON） | ✅ | ✅ | ✅ |
| 带分类预览和能力状态的仪表板 | ✅ | ✅ | ✅ |
| 搜索 116 个罗技设备 ID（固定版本的 libratbag 目录） | ✅ | ✅ | ✅ |
| 系统 USB/PnP 清单（摄像头、方向盘等） | ✅ | ✅ | ✅ |
| 设置 DPI 和回报率 | 🧪 HID++ 2.0 有线鼠标 | — | 🧪 通过 `ratbagd` |
| LED 0 常亮颜色 | — | — | 🧪 需手动开启，通过 `ratbagd` |

🧪 = 实验性功能，尚未在实机上验证。

RGB **默认关闭**，以便由 [rJb](https://github.com/jeremdevgit/rjb) 负责灯光。宏、按键映射、LIGHTSYNC 灯效、音频、摄像头、方向盘、固件、游戏检测和自动切换配置文件均未实现。设备出现在清单中并不代表可以配置。

## 安装

在[版本页面](https://github.com/jeremdevgit/lj-hub/releases)下载最新预发布版：Windows 为按用户安装的 `.exe`，macOS 为 Apple Silicon `.dmg`，Linux 为 Ubuntu/Debian x64 的 `.deb`。Windows 和 macOS 安装包未使用受信任的发布者证书签名。Linux 上的硬件控制需要 `ratbagd`、`ratbagctl` 以及 libratbag 支持的设备。

## 安全与隐私

- **不发起网络请求，无需账号。** 配置文件保存在系统的用户配置目录中。
- 硬件写入需手动开启，会对照设备声明的值进行检查并在写入后回读；只有经过测试的设备才会逐一开放。

## 路线图

1. 在各系统的实机上验证 HID++ 能力。
2. 为 DPI、回报率、RGB 和板载配置建立经过测试的能力表。
3. 按键映射和受限的宏引擎。
4. 在协议有文档的前提下支持耳机、麦克风、摄像头和方向盘。
5. 按游戏切换配置文件。

## 从源码构建

```sh
# Rust stable。Linux 还需要：libudev-dev libx11-dev libxi-dev libxcursor-dev
# libxrandr-dev libxkbcommon-dev libwayland-dev pkg-config
cargo run --locked --release
```

设备目录生成自 [libratbag 的设备文件](https://github.com/libratbag/libratbag/tree/master/data/devices)，版本固定在 `data/libratbag-logitech.json` 记录的提交；更新方法：`python3 tools/refresh_libratbag_catalog.py /path/to/libratbag`。更新日志：[CHANGELOG.md](CHANGELOG.md)。

## 许可证

GPL-3.0-or-later，见 [LICENSE](LICENSE)。Logitech、G HUB 和 LIGHTSYNC 是其所有者的商标。

---

发现安全漏洞？请私下报告（见 [SECURITY.md](SECURITY.md)），不要发公开 issue。

## jeremdevgit 的其他自由软件

全部免费、开源、无广告。

**🛠️ 系统与硬件** — [Boîte à outils](https://github.com/jeremdevgit/boite-a-outils) · [rJb](https://github.com/jeremdevgit/rjb) · [Decklight](https://github.com/jeremdevgit/decklight)

**📒 效率与数据** — [Carnet de notes](https://github.com/jeremdevgit/carnet-rust) · [JMeca](https://github.com/jeremdevgit/jmeca) · [Suite Libre](https://jeremdevgit.github.io/suite-libre/)

**🎮 音乐与游戏** — [Pomme](https://github.com/jeremdevgit/pomme) · [Ludothèque](https://github.com/jeremdevgit/ludotheque) · [AutoDex](https://github.com/jeremdevgit/autodex) · [Batailles de France](https://github.com/jeremdevgit/batailles-de-france)
