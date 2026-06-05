# SmartLedger 🧾

AI智能记账助手 - Rust + Kotlin 混合开发的 Android 应用

[![Build APK](https://github.com/Bingnnvjn/smart-ledger/actions/workflows/build.yml/badge.svg)](https://github.com/Bingnnvjn/smart-ledger/actions/workflows/build.yml)

## ✨ 功能特性

- 📝 **快速记账** - 两步完成，简洁高效
- 🤖 **AI智能** - 自然语言记账，智能分类
- 📊 **数据报表** - 可视化图表，清晰直观
- 💰 **预算管理** - 按分类设置，实时提醒
- 🔒 **本地存储** - 数据安全，隐私保护

## 🏗️ 技术架构

| 层级 | 技术 | 说明 |
|------|------|------|
| UI层 | Kotlin + Jetpack Compose | 原生Android界面 |
| 桥接层 | JNI | Kotlin ↔ Rust 通信 |
| 逻辑层 | Rust | 业务逻辑、AI、数据 |
| 网络层 | reqwest + tokio | MiMo API调用 |

## 📦 项目结构

```
smart-ledger/
├── android/          # Android 项目
├── rust/             # Rust 核心库
├── .github/          # GitHub Actions
└── docs/             # 文档
```

## 🚀 自动构建

推送到 `main` 分支会自动触发 GitHub Actions 构建 APK。

从 [Releases](https://github.com/Bingnnvjn/smart-ledger/releases) 页面下载最新 APK。

## 📄 License

MIT License
