<script>
    export default {
        data() {
            return {
                servers: [
                    {name: "server01", ping: 21, down: false},
                    {name: "server02", ping: 29, down: false},
                    {name: "server03", ping: 26, down: false},
                    {name: "server04", ping: 25, down: false},
                    {name: "server05", ping: 0,  down: true },
                    {name: "server06", ping: 43, down: false},
                    {name: "server07", ping: 21, down: false},
                    {name: "server08", ping: 27, down: false},
                    {name: "server09", ping: 22, down: false},
                    {name: "server10", ping: 56, down: false},
                    {name: "server11", ping: 22, down: false},
                    {name: "server12", ping: 21, down: false},
                    {name: "server13", ping: 29, down: false},
                    {name: "server14", ping: 26, down: false},
                    {name: "server15", ping: 25, down: false},
                    {name: "server16", ping: 0, down: true},
                    {name: "server17", ping: 43, down: false},
                    {name: "server18", ping: 21, down: false},
                    {name: "server19", ping: 27, down: false},
                    {name: "server20", ping: 22, down: false},
                    {name: "server21", ping: 56, down: false},
                    {name: "server22", ping: 22, down: false},
                ]
            };
        },
    };
</script>

<template>
    <div class="servers">
        <h2>servers</h2>
        <div class="server" v-for="{name, ping, down} in servers">
            <div class="name danger" v-if="down">{{ name }}</div>
            <div class="ping danger" v-if="down">down</div>
            <div class="name" v-if="!down">{{ name }}</div>
            <div class="ping warn" v-if="ping > 60">{{ ping }}ms</div>
            <div class="ping" v-if="ping <= 60 && !down">{{ ping }}ms</div>
        </div>
    </div>
</template>

<style>
    h2 {
        margin: 10px 5px;
    }
    .servers {
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
    .server {
        display: flex;
        justify-content: space-between;
        border-bottom: 3px solid var(--divider-color);
    }
    .server .name {
        letter-spacing: -4px;
        font-size: 32px;
    }
    .server .ping {
        width: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 20px;
    }
    .server .warn { color: #f8ac47 }
    .server .danger { color: #f84d47 }
</style>