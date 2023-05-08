<script>
    import axios from "/js/libs/axios.min.js";
    import Cookies from "/js/libs/jscookie.min.js";

    export default {
        data() {
            return {
                interval: null,
                reminders: []
            }
        },
        mounted() {
            this.interval = setInterval(() => {
                let token = Cookies.get("auth")
                axios
                .get("/api/reminders/active", {
                    headers: {
                        'Authorization': `Bearer ${token}`
                    }
                })
                .then(response => {
                    if (response.status == 200) {
                        const data = response.data;
                        this.reminders = [] 
                        data.forEach(reminder => {
                            this.reminders.push(reminder)
                        });
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
                });
            }, 5000)
            let token = Cookies.get("auth")
        }
    } 
</script>

<template>
    <div class="reminders">
        <h2>reminders</h2>
        <div class="reminder" v-for="{reminder, user_initials} in reminders">
            <div class="title">{{ reminder }}</div>
            <div class="owner">{{ user_initials.toLowerCase() }}</div>
        </div>
    </div>
</template>

<style>

    h2 {
        margin: 10px 5px;
    }

    .reminders {
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
    .reminder {
        display: flex;
        justify-content: space-between;
        padding: 5px 0px;
        border-bottom: 3px solid var(--divider-color);
    }
    .reminder .title {
        font-family: 'Raleway', sans-serif;
        width: 260px;
        font-size: 20px;
        color: var(--text-color);
    }
    .reminder .owner {
        width: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 16px;
    }
</style>