<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

function passObject (){
  // JS側は構造体にファジーなのでRust側の型設定が重要
  invoke('command_with_object', { message: { field_str: 'some message', field_u32: 12 }})
    .then(message => {
      console.log('command_with_object', message)
    })
}
function returnResult() {
  // Reslt が返ってきた場合 Ok(t) の t が then に渡され、Err(t)のtがcatchに渡される
  for (let arg of [1, 2]) {
    invoke('command_with_error', { arg })
      .then(message => {
        console.log('command_with_error', message)
      })
      .catch(message => {
        console.error('command_with_error', message)
      })
  }
}

</script>

<template>
  <div class="card">
    <button type="button" @click="passObject()">Rust ⇔ vueの構造体の送受信</button>
    <button type="button" @click="returnResult()">Rust ⇒ vueにResultの送信</button>
  </div>
</template>
