<script setup lang="ts">
import { emit } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { onMounted,ref } from 'vue'

const receiveState = ref(`never received`)
const receiveTime = ref(`never received`)

onMounted(async() =>{
  await listen('interrupt-from-back', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    receiveState.value = event.payload as string
    receiveTime.value = new Date().toISOString()
  })
})

function emitMessage() {
    emit('interrupt-to-back', {
      theMessage: 'Tauri is awesome!',
    })
}

</script>

<template>
  <div class="card">
    <button type="button" @click="emitMessage()">Rustへの割り込み</button>
    <p>受信時間：{{ receiveTime }}：、状態：{{ receiveState }}</p>
  </div>
</template>
