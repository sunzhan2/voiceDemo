<script setup lang="ts">
import VoiceRecorder from "./components/VoiceRecorder.vue"
</script>

<template>
  <div class="app-container">
    <VoiceRecorder />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "PingFang SC",
    "Microsoft YaHei", sans-serif;
  background: #f0f2f5;
  color: #333;
  -webkit-font-smoothing: antialiased;
}

.app-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: 20px;
}
</style>
