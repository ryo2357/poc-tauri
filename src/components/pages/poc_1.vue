<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "../parts/Greet.vue";
import DataCommunication from "../parts//DataCommunication.vue";
import TauriApi from "../parts/TauriApi.vue";
</script>

<template>
  <div class="container">
    <h1>Tauri機能検証　１</h1>

    <h2>Vue ⇒ Rust の invoke</h2>
    <Greet />

    <h2>データ構造体の通信</h2>
    <DataCommunication />

    <h2>Tauri APIの使用</h2>
    <TauriApi />

    <h2>リンク</h2>
    <div class="card">
      <router-link to="/poc2">Tauri機能検証　２</router-link>
    </div>

  </div>
</template>

<style scoped>
</style>
