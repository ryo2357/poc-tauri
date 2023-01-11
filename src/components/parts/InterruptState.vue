<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { onMounted,ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const receiveValue = ref(0)
const receiveTime = ref(`never received`)

onMounted(async() =>{
  await listen('interrupt-from-state', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    receiveValue.value = event.payload as number
    receiveTime.value = new Date().toISOString()
  })
})

function startProcess() {
    invoke('start_state_process')
      .then(message => {
        console.log('success start_state_process', message)
      })
      .catch(message => {
        console.error('error start_state_process', message)
      })
}

function stopProcess() {
    invoke('stop_state_process')
      .then(message => {
        console.log('success stop_state_process', message)
      })
      .catch(message => {
        console.error('error stop_state_process', message)
      })
}


</script>

<template>
  <div class="card">
    <button type="button" @click="startProcess()">スレッドの開始</button>
    <button type="button" @click="stopProcess()">スレッドの終了</button>
    <p>受信時間{{ receiveTime }}：、状態：{{ receiveValue }}</p>
  </div>
</template>
