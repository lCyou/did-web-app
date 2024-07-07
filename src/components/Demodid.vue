<script setup lang="ts">

const getKey = () => {
    window.__TAURI__
        .invoke('print_command')
}
</script>

<template>
    <div>
        <h1>Generate a key pair</h1>
        <button value="" @click="getKey"> get keys </button>
    </div>
</template>

<style>
</style>