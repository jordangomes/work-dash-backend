<script>
    import axios from "../libs/axios.min.js";
    export default {
        data() {
            return {
                interval: null,
                servers: []
            }
        },
        mounted() {
            this.interval = setInterval(() => {
                let token = Cookies.get("auth")
                axios
                .get("/api/ping", {
                    headers: {
                        'Authorization': `Bearer ${token}`
                    }
                })
                .then(response => {
                    if (response.status == 200) {
                        const data = response.data;
                        console.log(data)
                        this.servers = [] 
                        data.forEach(rawping => {
                            let ping = {};
                            ping.name = rawping.label;
                            ping.ping = rawping.ping;
                            ping.down = (rawping.error == "") ? false : true

                            this.servers.push(ping)
                        });
                    } else if (response.status == 401) {
                        document.location.href="/login";
                    }
                }).catch(function (error) {
                    if (error.response) {
                        if(error.response.status ==401) {
                            document.location.href="/login";
                        }
                    console.log(error.response.data);
                    console.log(error.response.status);
                    console.log(error.response.headers);
                    }
                });;
            }, 5000)
        }
    } 
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
        text-transform: lowercase;
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