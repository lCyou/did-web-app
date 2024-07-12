<script setup lang="ts">
function print_command(){
    window.__TAURI__
        .invoke('create_keypair')
}

</script>

<template>
    <div>
        <h1>Generate a key pair</h1>
        <button value="" @click="print_command"> get keys </button>
    </div>
</template>

<style>
</style>