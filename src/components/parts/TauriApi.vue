<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog'

function openDialog(){
  open().then(files => console.log(files))
}
</script>

<template>
  <div class="card">
    <button type="button" @click="openDialog()">ファイルダイアログの使用</button>
  </div>
</template>
