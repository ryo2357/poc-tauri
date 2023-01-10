<script setup lang="ts">
import { emit } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { onMounted,ref } from 'vue'

const receiveValue = ref(0)
const receiveTime = ref(`never received`)

onMounted(async() =>{
  await listen('interrupt-from-struct', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    receiveValue.value = event.payload as number
    receiveTime.value = new Date().toISOString()
  })
})

function startProcess() {
    emit('start-struct-process')
}

function stopProcess() {
    emit('stop-struct-process')
}

</script>

<template>
  <div class="card">
    <button type="button" @click="startProcess()">処理の開始</button>
    <button type="button" @click="stopProcess()">処理の停止</button>
    <p>受信時間{{ receiveTime }}：、状態：{{ receiveValue }}</p>
  </div>
</template>
