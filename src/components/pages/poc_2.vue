<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Interrupt from "../parts/Interrupt.vue";
// import InterruptStruct from "../parts/InterruptStruct.vue";
// import InterruptState from "../parts/InterruptState.vue";
// import tokioThread from "../parts/tokioThread.vue";
</script>

<template>
  <div class="container">
    <h1>Tauri機能検証　２</h1>

    <h2>Rust ⇔ Vue の 一方向通信</h2>
    <Interrupt />

    <h2>tokioスレッドとの通信</h2>
    <tokioThread />

    <!-- <h2>構造体によるemitスレッドの管理</h2>
    <InterruptStruct /> -->

    <!-- <h2>Stateにスレッドを持たせる</h2>
    <InterruptState /> -->


    <h2>リンク</h2>
    <div class="card">
      <router-link to="/poc1">Tauri機能検証　１</router-link>
    </div>

  </div>
</template>

<style scoped>
</style>
