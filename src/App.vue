<template>
    <!-- <el-header style="height: 30px"><AppHeader ref="element"></AppHeader></el-header> -->
    <el-main>
        <router-view />
    </el-main>
    <!-- <el-footer> </el-footer> -->
</template>

<script lang="ts" setup>
import AppHeader from './components/AppHeader.vue'
import { invoke } from '@tauri-apps/api/core'

// document.addEventListener('DOMContentLoaded', () => {
//     //关闭loading窗口
//     setTimeout(() => {
//         invoke('loading_window_close')
//     }, 3000)
// })
</script>

<style>
body {
    margin: 0;
}
.el-main {
    margin: 0;
    height: 100vh;
    font-family: 'Avenir', Helvetica, Arial, sans-serif;
    background-color: rgb(244, 244, 245);
}
</style>
