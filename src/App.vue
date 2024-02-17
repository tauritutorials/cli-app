<template>
    <div class="container">Hello, {{ message }}</div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";

const message = ref();

onMounted(async () => {
    message.value = await invoke("get_name");
});
</script>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
