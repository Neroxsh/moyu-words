# 摸鱼背词 (Moyu Words)

一款可在本地运行、可打包成 macOS App 和 Windows exe 的摸鱼背单词桌面应用。

**技术栈**: Tauri v2 + React + TypeScript + Rust + SQLite

## 功能

- 本地词书导入：支持 `.txt`、`.csv`、`.json` 格式
- 内置词书：初中、高中、四级、六级、考研、托福、SAT
- 学习计划：按"几天背完"自动切分单元
- 摸鱼模式：透明、置顶、**可拖拽缩放**的悬浮窗
- 鼠标控制：左键上一词，右键下一词，**左键长按 3 秒触发文字碎裂动画 → 归档熟识单词**
- 键盘控制：`Ctrl+Left` / `Ctrl+Right` 切词，空格暂停/继续
- 自动切词：可设置间隔秒数
- 熟识词归档：已掌握的单词自动移出背诵队列
- 进度管理：单元进度、打卡、熟识词查看、恢复、导出

## 开发

### 前置环境

- [Node.js 20+](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/tools/install)
- macOS: Xcode Command Line Tools (`xcode-select --install`)
- Windows: [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)（Win10+ 已预装）

### 本地运行

```bash
npm install
npm run tauri dev
```

### 打包

```bash
npm run tauri build
```

产物：
- macOS: `src-tauri/target/release/bundle/dmg/`
- Windows: `src-tauri/target/release/bundle/msi/`

## 项目结构

```text
moyu-words/
├── src/                    # React 前端
│   ├── main.tsx            # 主窗口入口
│   ├── overlay.tsx         # 摸鱼窗口入口
│   ├── App.tsx             # 根组件
│   ├── components/         # UI 组件
│   ├── hooks/              # 自定义 hooks
│   ├── store/              # Zustand 状态管理
│   ├── lib/                # API 封装 + 工具函数
│   └── types/              # TypeScript 类型
├── src-tauri/              # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── resources/vocab/    # 内置词书
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── db/             # SQLite 数据库模块
│       ├── vocab/          # 词书解析
│       └── commands/       # Tauri 命令
├── package.json
├── index.html              # 主窗口 HTML
├── overlay.html            # 摸鱼窗口 HTML
└── vite.config.ts
```

## 词书来源

内置词书整理自 [`KyleBing/english-vocabulary`](https://github.com/KyleBing/english-vocabulary)。

## License

MIT