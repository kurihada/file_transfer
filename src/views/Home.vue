<template>
    <div class="home-container">
        <div class="home-center">
            <el-row justify="center" align="middle" :gutter="20">
                <el-text size="large" style="margin: 10px">File Transfer</el-text>

                <el-col justify="center" align="middle">
                    <el-radio-group v-model="isCollapse" style="margin-bottom: 20px">
                        <el-radio-button :value="true">发送文件</el-radio-button>
                        <el-radio-button :value="false">接收文件</el-radio-button>
                    </el-radio-group>
                </el-col>

                <el-row v-show="isCollapse" justify="center" align="middle" :gutter="20">
                    <el-col :span="12" align="end">
                        <el-button type="primary" @click="scanDevices">Scan Devices</el-button>
                    </el-col>
                    <el-col :span="12">
                        <el-select
                            v-model="selectedDevice"
                            placeholder="Select a device"
                            style="width: 10vw"
                        >
                            <el-option
                                v-for="device in devices"
                                :key="device.name"
                                :label="device.name"
                                :value="device.name"
                            ></el-option>
                        </el-select>
                    </el-col>

                    <el-col :span="24" :offset="0" align="middle" style="height: 30px"></el-col>

                    <el-col align="middle">
                        <el-upload
                            v-model:file-list="fileList"
                            class="upload-demo"
                            multiple
                            :on-preview="handlePreview"
                            :on-remove="handleRemove"
                            :before-remove="beforeRemove"
                            :limit="3"
                            :on-exceed="handleExceed"
                            :on-success="handleSuccess"
                        >
                            <el-button type="primary">Click to upload</el-button>
                            <template #tip>
                                <div class="el-upload__tip">
                                    jpg/png files with a size less than 500KB.
                                </div>
                            </template>
                        </el-upload>
                    </el-col>

                    <el-col :span="24" :offset="0" align="middle" style="height: 30px"></el-col>

                    <el-col align="middle">
                        <el-button type="primary" @click="transferFile">Transfer File</el-button>
                    </el-col>

                    <el-col align="middle" style="margin: 30px">
                        <el-text size="large">Devices found.</el-text>
                    </el-col>

                    <el-input
                        v-model="logText"
                        style="width: 20vw"
                        :autosize="{ minRows: 6 }"
                        type="textarea"
                        placeholder="Waiting"
                        readonly="true"
                    />
                </el-row>

                <el-row v-show="!isCollapse"> 接收文件 </el-row>
                <el-col :span="24" :offset="0" style="height: 30px"></el-col>
            </el-row>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
    ElMessage,
    ElMessageBox,
    UploadFile,
    UploadFiles,
    UploadProps,
    UploadUserFile,
} from 'element-plus'

interface DeviceInfo {
    name: string
    ip: string
    kind: DeviceKind
}

enum DeviceKind {
    Window,
    Linux,
    Android,
}

const isCollapse = ref(true)
const operation = ref('send')
let devices = ref<DeviceInfo[]>([])
const selectedDevice = ref('')
const logText = ref('')
const fileList = ref<UploadUserFile[]>([])

async function scanDevices() {
    devices.value = await invoke('scan_devices')
    logText.value = 'found:' + devices.value.map((device) => device.name).join('\n')
}

const transferFile = () => {
    if (operation.value === 'send') {
        console.log('Sending file to', selectedDevice.value)
    } else {
        console.log('Receiving file from', selectedDevice.value)
    }
}

const handleRemove: UploadProps['onRemove'] = (file, uploadFiles) => {
    console.log(file, uploadFiles)
}

const handlePreview: UploadProps['onPreview'] = (uploadFile) => {
    console.log(uploadFile)
}

const handleExceed: UploadProps['onExceed'] = (files, uploadFiles) => {
    ElMessage.warning(
        `The limit is 3, you selected ${files.length} files this time, add up to ${
            files.length + uploadFiles.length
        } totally`
    )
}

const beforeRemove: UploadProps['beforeRemove'] = (uploadFile, uploadFiles) => {
    return ElMessageBox.confirm(`Cancel the transfer of ${uploadFile.name} ?`).then(
        () => true,
        () => false
    )
}

const handleSuccess: UploadProps['onSuccess'] = (
    response: any,
    uploadFile: UploadFile,
    uploadFiles: UploadFiles
) => {
    console.log(response, uploadFile, uploadFiles)
}
</script>

<style scoped>
.home-container {
    display: flex;
    justify-content: center;
    align-items: center;
}
.home-center {
    display: grid;
    place-items: center;
    background-color: rgb(255, 255, 255);
    height: 80vh;
    width: 40vw;
    border-radius: 10px;
    box-shadow: 7px 7px 12px rgba(0, 0, 0, 0.4), -7px -7px 12px rgba(255, 255, 255, 0.9);
}
</style>
