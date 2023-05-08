<script>
    import axios from "../libs/axios.min.js";
    import Cookies from "/js/libs/jscookie.min.js";

    export default {
        data() {
            return {
                interval: null,
                feed: []
            }
        },
        mounted() {
            this.interval = setInterval(() => {
                let token = Cookies.get("auth")
                axios
                .get("/api/rss/feed", {
                    headers: {
                        'Authorization': `Bearer ${token}`
                    }
                })
                .then(response => {
                    if (response.status == 200) {
                        const data = response.data;
                        this.feed = [] 
                        data.forEach(feeditem => {
                            let item = {};

                            item.source_label = feeditem.source_label
                            item.title = feeditem.title
                            item.important = feeditem.important && !feeditem.dismissed ? "important" : ""

                            this.feed.push(item)
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
    <div class="feed">
        <h2>feed</h2>
        <div class="article" v-for="{title, source_label, important} in feed">
            <div :class="['source_label', important]">{{ source_label }}</div>
            <div :class="['title', important]">{{ title }}</div>
        </div>
    </div>
</template>

<style>

    h2 {
        margin: 10px 5px;
    }

    .feed {
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
    .article {
        /* display: flex;
        justify-content: space-between; */
        padding: 10px 0px;
        border-bottom: 3px solid var(--divider-color);
    }
    .article .title.important {
        color: #f84d47
    }
    .article .source_label.important {
        color: #f84d47
    }
    .article .title {
        font-family: 'Raleway', sans-serif;
        font-size: 18px;
        color: var(--text-color);
    }
    .article .source_label {
        padding-bottom: 5px;
        font-size: 16px;
        /* text-transform: lowercase; */
    }
</style>