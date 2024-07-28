<template>
    <!-- <a-layout-header> -->
    <div class="header" data-tauri-drag-region>
        <!-- <div class="left">
            <span><a-image :width="24" :src="logo" :preview="false" /> </span>
            <span></span>
        </div> -->
        <div class="right">
            <span
                ><Edit style="width: 1em; height: 1em; margin-right: 8px" @click="openAboutWin"
            /></span>
            <span
                ><Edit style="width: 1em; height: 1em; margin-right: 8px" @click="handleWinMin"
            /></span>
            <span
                ><Edit style="width: 1em; height: 1em; margin-right: 8px" @click="handleWinMax2Min"
            /></span>
            <span
                ><Edit style="width: 1em; height: 1em; margin-right: 8px" @click="showModal"
            /></span>
        </div>
    </div>
</template>
<script>
import { defineComponent } from 'vue'
export default defineComponent({
    components: {},
    props: {
        title: {
            type: String,
            default: 'aaa',
        },
        WinMin: {
            type: Boolean,
            default: false,
        },
        WinClose: {
            type: Boolean,
            default: true,
        },
        WinMax: {
            type: Boolean,
            default: false,
        },
        WinIcon: {
            type: Boolean,
            default: false,
        },
        iconType: {
            type: String,
            default: '',
        },
    },
    setup() {
        // 最小化
        const handleWinMin = async () => {}
        //后台运行
        const handleCancel = async () => {}

        // 最大化/还原
        const handleWinMax2Min = async () => {}
        //关闭
        const handleWinClose = async () => {}

        return {
            handleWinClose,
            handleWinMin,
            handleWinMax2Min,
            handleCancel,
        }
    },
})
</script>
<style scoped>
.header {
    height: 30px;
    display: flex;
    /*align-items 属性定义flex子项在flex容器的当前行的侧轴（纵轴）方向上的对齐方式。 */
    align-items: center;
    /* 左右边距 */
    padding: 0 3px;
    line-height: normal;
    max-height: 30px;
    background-color: rgb(223, 223, 223);

    justify-content: space-between;
    -webkit-app-region: drag;
}

.right span {
    margin-left: 10px;
    margin-right: 5px;
    -webkit-app-region: no-drag;
}
.left {
    width: 30%;
    display: flex;
    justify-content: left;
}
.center {
    width: 40%;
    display: flex;
    justify-content: center;
}
.right {
    width: 30%;
    display: flex;
    justify-content: right;
}
</style>
