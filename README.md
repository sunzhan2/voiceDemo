# VoiceDemo - Tauri + Rust + SenseVoice 离线语音转文字

## 功能

- 跨平台麦克风实时音频采集
- SenseVoice-Small ONNX 模型本地推理
- 录音启停控制
- 纯离线运行，保护隐私

## 环境要求

- Rust 1.65+
- Node.js 16+ / npm
- Windows：Visual Studio Build Tools（C++ 桌面开发）

## 快速开始

```bash
# 1. 安装 Rust（如未安装）
# 访问 https://rustup.rs 下载安装

# 2. 安装依赖
npm install

# 3. 下载 SenseVoice 模型
# 将 sensevoice.onnx 放入 src-tauri/models/ 目录

# 4. 开发模式运行
npm run tauri dev

# 5. 生产构建
npm run tauri build
```

## 项目结构

```
voice-demo/
├── src/                    # Vue 前端
│   ├── App.vue
│   ├── main.ts
│   └── components/
│       └── VoiceRecorder.vue
├── src-tauri/              # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs         # 入口
│       ├── audio.rs        # 音频采集 (cpal)
│       ├── inference.rs    # ONNX 推理 (ort)
│       └── commands.rs     # Tauri 命令
├── index.html
├── package.json
└── vite.config.ts
```

## 模型下载

SenseVoice-Small ONNX 模型需自行下载放置到 `src-tauri/models/`：

- 官方模型仓库：https://github.com/FunAudioLLM/SenseVoice

## 许可证

MIT
