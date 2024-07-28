<template>
    <div>
        <p>Devices found.</p>
        <ul>
            <li v-for="device in devices" :key="device">{{ device }}</li>
        </ul>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue'

export default defineComponent({
    name: 'DeviceList',
    props: {
        devices: {
            type: Array as PropType<string[]>,
            required: true,
        },
    },
})
</script>
