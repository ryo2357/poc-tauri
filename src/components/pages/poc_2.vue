<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Interrupt from "../parts/Interrupt.vue";
</script>

<template>
  <div class="container">
    <h1>Tauri機能検証　２</h1>

    <h2>Rust ⇔ Vue の 一方向通信</h2>
    <Interrupt />


    <h2>リンク</h2>
    <div class="card">
      <router-link to="/poc1">Tauri機能検証　１</router-link>
    </div>

  </div>
</template>

<style scoped>
</style>
