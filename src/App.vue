<template>
    <el-main>
        <router-view />
    </el-main>
</template>

<script lang="ts" setup></script>
<style>
/* 全局样式 */
html,
body {
    -webkit-user-select: none; /* Safari */
    -moz-user-select: none; /* Firefox */
    -ms-user-select: none; /* Internet Explorer/Edge */
    user-select: none; /* Non-prefixed version, currently supported by Chrome, Opera, and Edge */
}

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
