<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/tauri"

const isRecording = ref(false)
const transcript = ref("")
const statusText = ref("就绪")
const isLoading = ref(true)
const errorMessage = ref("")

onMounted(async () => {
  try {
    const modelDir = await invoke<string>("get_model_dir")
    await invoke("load_model", { modelDir })
    statusText.value = "模型已加载，点击录音按钮开始"
    isLoading.value = false
  } catch (e: any) {
    errorMessage.value = `模型加载失败: ${e}`
    isLoading.value = false
  }
})

async function toggleRecording() {
  if (isRecording.value) {
    await stopRecording()
  } else {
    await startRecording()
  }
}

async function startRecording() {
  try {
    statusText.value = "正在录音..."
    await invoke("start_recording")
    isRecording.value = true
  } catch (e: any) {
    statusText.value = "录音启动失败"
    errorMessage.value = `错误: ${e}`
  }
}

async function stopRecording() {
  try {
    const result = await invoke<string>("stop_recording")
    isRecording.value = false
    statusText.value = "录音已停止"
    transcript.value = result
  } catch (e: any) {
    statusText.value = "停止录音失败"
    errorMessage.value = `错误: ${e}`
  }
}
</script>

<template>
  <div class="voice-recorder">
    <div v-if="isLoading" class="loading">
      <div class="spinner"></div>
      <p>正在加载语音模型...</p>
    </div>

    <div v-else class="content">
      <div class="status-bar">
        <span
          class="status-indicator"
          :class="{ recording: isRecording }"
        ></span>
        <span class="status-text">{{ statusText }}</span>
      </div>

      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
      </div>

      <button
        class="record-btn"
        :class="{ recording: isRecording }"
        @click="toggleRecording"
        :disabled="isLoading"
      >
        <span class="btn-icon">{{ isRecording ? "■" : "●" }}</span>
        <span class="btn-text">{{ isRecording ? "停止录音" : "开始录音" }}</span>
      </button>

      <div class="transcript-box">
        <label class="transcript-label">识别结果</label>
        <div class="transcript-content">
          {{ transcript || "等待录音..." }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.voice-recorder {
  width: 100%;
  max-width: 420px;
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.08);
  padding: 32px 24px;
}

.loading {
  text-align: center;
  padding: 40px 0;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #e8e8e8;
  border-top-color: #4a90d9;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  justify-content: center;
}

.status-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #bbb;
  transition: background 0.3s;
}

.status-indicator.recording {
  background: #e74c3c;
  animation: pulse 1.2s ease-in-out infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.4;
  }
}

.status-text {
  font-size: 14px;
  color: #666;
}

.error-message {
  background: #fff0f0;
  color: #c0392b;
  padding: 10px 14px;
  border-radius: 8px;
  font-size: 13px;
  word-break: break-all;
}

.record-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 14px 24px;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  cursor: pointer;
  transition: all 0.3s;
  background: #4a90d9;
  color: #fff;
}

.record-btn:hover {
  background: #357abd;
}

.record-btn.recording {
  background: #e74c3c;
  animation: btnPulse 1.5s ease-in-out infinite;
}

.record-btn.recording:hover {
  background: #c0392b;
}

.record-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

@keyframes btnPulse {
  0%,
  100% {
    box-shadow: 0 0 0 0 rgba(231, 76, 60, 0.4);
  }
  50% {
    box-shadow: 0 0 0 8px rgba(231, 76, 60, 0);
  }
}

.btn-icon {
  font-size: 20px;
  line-height: 1;
}

.transcript-box {
  background: #f8f9fa;
  border-radius: 12px;
  padding: 16px;
}

.transcript-label {
  display: block;
  font-size: 12px;
  color: #999;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.transcript-content {
  font-size: 15px;
  line-height: 1.6;
  color: #333;
  min-height: 48px;
  word-break: break-all;
}
</style>
