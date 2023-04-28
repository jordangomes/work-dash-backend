<script>
    export default {
        data() {
            return {
                temps: [
                    {location: "l17", temp: "30", status: "danger"},
                    {location: "l18", temp: "22", status: "normal"},
                    {location: "l19", temp: "26", status: "warn"},
                ]
            };
        }
    };
</script>

<template>
    <div class="temps">
        <h2>comms rooms</h2>
        <div class="temp" v-for="{location, temp, status} in temps">
            <div :class="['temprature', status]">{{ temp }}ºC</div>
            <div class="location">{{ location }}</div>
        </div>
    </div>
</template>

<style>

    h2 {
        margin: 10px 5px;
    }

    .temps {
        -ms-grid-column-span: 2;
        position: relative;
        padding: 20px;
        width: calc(100% - 40px);
        height: calc(100% - 40px);
        border-radius: 20px;
        background-color: var(--background-color);
        color: var(--text-color);
        font-family: 'Major Mono Display', monospace;
    }
    .temp {
        display: flex;
        justify-content: space-between;
        border-bottom: 3px solid var(--divider-color);
    }
    .temp .temprature {
        letter-spacing: -4px;
        font-size: 48px;
        color: var(--text-color);
    }

    .temp .temprature.normal { color: #ffffff }
    .temp .temprature.warn { color: #f8ac47 }
    .temp .temprature.danger { color: #f84d47 }
    .temp .location {
        width: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 20px;
    }
</style>