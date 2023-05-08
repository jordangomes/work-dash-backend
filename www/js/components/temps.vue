<script>
    import axios from "../libs/axios.min.js";
    export default {
        data() {
            return {
                interval: null,
                temps: []
            }
        },
        mounted() {
            this.interval = setInterval(() => {
                let token = Cookies.get("auth")
                axios
                .get("/api/temperatures", {
                    headers: {
                        'Authorization': `Bearer ${token}`
                    }
                })
                .then(response => {
                    if (response.status == 200) {
                        const data = response.data;
                        this.temps = [] 
                        data.forEach(rawtemp => {
                            let temp = {};

                            temp.location = rawtemp.label
                            temp.temp = rawtemp.temp

                            let online = (rawtemp.last_set_time > (Math.floor(Date.now() / 1000) - 60))
                            if (!online) { 
                                temp.status = "offline" 
                            } else if (temp.temp >= 28) { 
                                temp.status = "danger" 
                            } else if(temp.temp >= 26) { 
                                temp.status = "warn" 
                            } else { 
                                temp.status = "normal" 
                            }
                            this.temps.push(temp)
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
    <div class="temps">
        <h2>comms rooms</h2>
        <div class="temp" v-for="{location, temp, status} in temps">
            <div :class="['temprature', status]">
                <span v-if="status == 'offline'">offline</span>
                <span v-else>{{ temp }}ºC</span>
            </div>
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
    .temp .temprature.offline { color: #f84d47 }

    .temp .location {
        width: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 20px;
    }
</style>