<script>
import axios from "/js/libs/axios.min.js";
import Cookies from "/js/libs/jscookie.min.js";


export default {
    data() {
        return {
            username: "",
            password: ""
        }
    },
    methods: {
        login() {
            axios
                .post("/api/login", { username: this.username, password: this.password })
                .then(response => {
                    if (response.status == 200) {
                        const token = response.data.token;
                        Cookies.set('auth', token, { expires: 1 })
                        document.location.href="/";
                    }
                });
        }
    }
};
</script>

<script setup>
</script>

<template>
    <div class="body">
        
        <div class="container">
            <main>
                <div class="login">
                    <h2>login</h2>
                    <form @submit.self.prevent="login">
                        <label for="username">username</label>
                        <input name="username" type="text" v-model="username">
                        <br/>
                        <label for="password">password</label>
                        <input name="password" type="password" v-model="password">
                        <br/>
                        <button type="submit">submit</button>
                    </form>
                </div>
            </main>
        </div>
    </div>
</template>



<style>
    h2 {
        /* padding: px 5px; */
        margin: 10px 0px;
    }
    .container {
        position: relative;
        overflow: hidden;
        height: 100%;
    }
    .container main {
        display: flex;
        justify-content: center;
        /* height: calc(100vh - 100px); */
        padding: 40px 40px 40px 40px;
        color: var(--text-color);
        font-family: 'Major Mono Display', monospace;
    }
    .container main .login {
        width: 300px;
        height: auto;
        display: flex;
        flex-direction: column;
        padding: 60px;
        background-color: var(--background-color);
        border-radius: 20px;
    }

    .container main .login form {
        display: flex;
        flex-direction: column;
        /* flex */
    }

</style>