<template>
    <el-container style="height: 100%; background-color: aliceblue">
        <Header
            @handleWinMin="handleWinMin"
            @handleWinClose="handleWinClose"
            title="关于"
            ref="element"
        ></Header>
        <el-container ref="Conten" :style="{ height: `${height_content}px`, minHeight: 'hidden' }">
            <el-container style="padding: 0 0; height: 100%; overflow: hidden">
                <el-main :style="{ background: 'rgb(208 208 208)', padding: '0 0' }">
                    <div
                        :style="{
                            background: 'rgb(208 208 208)',
                            display: 'flex',
                            height: '100%',
                            width: '100%',
                            padding: '5px',
                        }"
                    >
                        <div
                            class="appinfo"
                            :style="{ background: 'lightgrey', height: '100%', width: '50%' }"
                        >
                            <p style="text-align: left">AppName: {{ data.appName }}</p>
                            <p style="text-align: left">TauriVersion: {{ data.tauriVersion }}</p>
                            <p style="text-align: left">AppVersion: {{ data.appVersion }}</p>
                            <p style="text-align: left">ArchName: {{ data.archName }}</p>
                            <p style="text-align: left">PlatformName: {{ data.platformName }}</p>
                            <p style="text-align: left">OsType: {{ data.osType }}</p>
                            <p style="text-align: left">OsVersion: {{ data.osVersion }}</p>
                            <p style="text-align: left">联系邮箱: 1475316789@qq.com</p>
                            <p style="text-align: left">
                                图标出处: <br />https://www.flaticon.com/free-icon/substance_9814620
                            </p>
                            <p style="text-align: left">
                                gitee地址: <br />https://gitee.com/wurongxun/md-note
                            </p>
                        </div>
                        <div :style="{ background: 'lightgrey', height: '100%', width: '50%' }">
                            <div style="text-align: center; color: red">请我喝杯咖啡</div>
                            <div>
                                <el-image
                                    :alt="'请我喝杯咖啡'"
                                    :style="{ width: '100%' }"
                                    :src="data.coffee"
                                />
                            </div>
                        </div>
                    </div>
                </el-main>
            </el-container>
        </el-container>
    </el-container>
</template>
<script>
import Header from '../components/AppHeader.vue'
import { defineComponent, ref, onMounted, computed, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app'
import { arch, platform, type, version } from '@tauri-apps/api/os'

export default defineComponent({
    components: {
        Header,
    },
    setup() {
        const data = reactive({
            appName: '',
            tauriVersion: '',
            appVersion: '',
            archName: '',
            platformName: '',
            osType: '',
            osVersion: '',
            coffee: ref(require('../assets/coffee.jpg')),
        })
        const handleWinMin = async () => {
            invoke('windows_hide', { label: 'about-window' }).then((res) => {
                console.log(res)
            })
        }
        const handleWinClose = async () => {
            invoke('windows_close', { label: 'about-window' }).then((res) => {
                console.log(res)
            })
        }

        const height_content = computed(() => {
            // 回调函数必须return，结果就是计算的结果
            // 如果计算属性依赖的数据发生变化，那么会重新计算
            return document.documentElement.clientHeight - 30
        })

        onMounted(() => {
            // var width = document.documentElement.clientWidth;
            // var height = document.documentElement.clientHeight;
        })
        const headerStyle = {
            textAlign: 'center',
            color: '#fff',
            height: 64,
            paddingInline: 50,
            lineHeight: '64px',
            backgroundColor: '#7dbcea',
        }

        onMounted(async () => {
            data.appName = await getName()
            data.tauriVersion = await getTauriVersion()
            data.appVersion = await getVersion()
            data.archName = await arch()
            data.platformName = await platform()
            data.osType = await type()
            data.osVersion = await version()
        })

        return {
            height_content,
            headerStyle,
            handleWinMin,
            handleWinClose,
            data,
        }
    },
})
</script>
<style scoped>
span {
    margin-top: 10px;
}

.appinfo p {
    margin-bottom: 10px;
}
</style>
