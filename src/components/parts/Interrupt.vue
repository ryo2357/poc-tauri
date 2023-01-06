<script setup lang="ts">
import { emit } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { onMounted } from 'vue'

onMounted(async() =>{
  await listen('back-to-front', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(`back-to-front ${event.payload} ${new Date()}`)
  })
})

function emitMessage() {
    console.log("test")
    // emit('front-to-back', "hello from front")
    emit('front-to-back', {
      theMessage: 'Tauri is awesome!',
    })
}

function startSignal(){
  emit('start-signal')
}

function stopSignal(){
  emit('stop-signal')
}


</script>

<template>
  <div class="card">
    <button type="button" @click="emitMessage()">Rustへの割り込み</button>
    <button type="button" @click="startSignal()">Rustからの一方向通信の開始</button>
    <button type="button" @click="stopSignal()">一方向通信の終了</button>
  </div>
</template>
