<template>
    <div class="file-transfer-form">
        <h1>File Transfer</h1>
        <el-radio-group v-model="isCollapse" style="margin-bottom: 20px">
            <el-radio-button :value="true">发送文件</el-radio-button>
            <el-radio-button :value="false">接收文件</el-radio-button>
        </el-radio-group>

        <el-button type="primary" @click="scanDevices">Scan Devices</el-button>
        <el-select class="custom-select" v-model="selectedDevice" placeholder="Select a device">
            <el-option
                v-for="device in devices"
                :key="device"
                :label="device"
                :value="device"
            ></el-option>
        </el-select>
        <el-upload
            v-if="operation === 'send'"
            action="http://localhost:3000/upload"
            :on-success="handleSuccess"
            :on-error="handleError"
        >
            <el-button type="primary">选择文件</el-button>
        </el-upload>
        <el-button type="primary" @click="transferFile">Transfer File</el-button>
        <div>
            <p>Devices found.</p>
            <ul>
                <li v-for="device in devices" :key="device">{{ device }}</li>
            </ul>
        </div>
    </div>
</template>

<script lang="ts" setup>
import ref from 'vue'
const isCollapse = ref(true)
const operation = ref('send')
const devices = ref<string[]>([])
const selectedDevice = ref('')

const scanDevices = () => {
    // Mock scanning devices
    devices.value = ['Device 1', 'Device 2', 'Device 3']
}

const handleSuccess = (response: any, file: any) => {
    console.log('File uploaded successfully:', file)
}

const handleError = (err: any, file: any) => {
    console.error('File upload failed:', file)
}

const transferFile = () => {
    if (operation.value === 'send') {
        console.log('Sending file to', selectedDevice.value)
    } else {
        console.log('Receiving file from', selectedDevice.value)
    }
}
</script>

<style scoped>
.file-transfer-form {
    background-color: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    max-width: 400px;
    width: 100%;
    text-align: center;
}
</style>
