<template>
    <div class="body">
        <!-- Loading -->
        <div class="preload">
            <!-- seven balls -->
            <div class="ball">L</div>
            <div class="ball">O</div>
            <div class="ball">A</div>
            <div class="ball">D</div>
            <div class="ball">I</div>
            <div class="ball">N</div>
            <div class="ball">G</div>
        </div>
    </div>
</template>
<script>
import { defineComponent} from 'vue';
export default defineComponent({
    setup() {

    },
});
</script>
<style scoped>
.body {
    height: 100vh;
    background: #191724;
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: center;
}

.preload {
    background: #191724;
    display: contents;
    padding: 5px;
}

.ball {
    width: 20px;
    height: 20px;
    margin: 20px auto;
    border-radius: 50px;
    text-align: center;
}

.ball:nth-child(1) {
    background: #ea9a97;
    -webkit-animation: right 1s infinite ease-in-out;
    -moz-animation: right 1s infinite ease-in-out;
    animation: right 1s infinite ease-in-out;
}

.ball:nth-child(2) {
    background: #3e8fb0;
    -webkit-animation: left 1.1s infinite ease-in-out;
    -moz-animation: left 1.1s infinite ease-in-out;
    animation: left 1.1s infinite ease-in-out;
}

.ball:nth-child(3) {
    background: #eb6f92;
    -webkit-animation: right 1.05s infinite ease-in-out;
    -moz-animation: right 1.05s infinite ease-in-out;
    animation: right 1.05s infinite ease-in-out;
}

.ball:nth-child(4) {
    background: #9ccfd8;
    -webkit-animation: left 1.15s infinite ease-in-out;
    -moz-animation: left 1.15s infinite ease-in-out;
    animation: left 1.15s infinite ease-in-out;
}

.ball:nth-child(5) {
    background: #c4a7e7;
    -webkit-animation: right 1.1s infinite ease-in-out;
    -moz-animation: right 1.1s infinite ease-in-out;
    animation: right 1.1s infinite ease-in-out;
}

.ball:nth-child(6) {
    background: #f6c177;
    -webkit-animation: left 1.05s infinite ease-in-out;
    -moz-animation: left 1.05s infinite ease-in-out;
    animation: left 1.05s infinite ease-in-out;
}

.ball:nth-child(7) {
    background: #b4637a;
    -webkit-animation: right 1s infinite ease-in-out;
    -moz-animation: right 1s infinite ease-in-out;
    animation: right 1s infinite ease-in-out;
}

@-webkit-keyframes right {
    0% {
        -webkit-transform: translate(-15px);
    }

    50% {
        -webkit-transform: translate(15px);
    }

    100% {
        -webkit-transform: translate(-15px);
    }
}

@-webkit-keyframes left {
    0% {
        -webkit-transform: translate(15px);
    }

    50% {
        -webkit-transform: translate(-15px);
    }

    100% {
        -webkit-transform: translate(15px);
    }
}

@-moz-keyframes right {
    0% {
        -moz-transform: translate(-15px);
    }

    50% {
        -moz-transform: translate(15px);
    }

    100% {
        -moz-transform: translate(-15px);
    }
}

@-moz-keyframes left {
    0% {
        -moz-transform: translate(15px);
    }

    50% {
        -moz-transform: translate(-15px);
    }

    100% {
        -moz-transform: translate(15px);
    }
}

@keyframes right {
    0% {
        transform: translate(-15px);
    }

    50% {
        transform: translate(15px);
    }

    100% {
        transform: translate(-15px);
    }
}

@keyframes left {
    0% {
        transform: translate(15px);
    }

    50% {
        transform: translate(-15px);
    }

    100% {
        transform: translate(15px);
    }
}
</style>
  
  