<script>
    export default {
        props: ['name', 'image', 'status']
    }
</script>
<template>
    <div :class="['status', status]">
        <div class="imgcontainer">
            <img :src="'/img/profiles/' + image" :alt="name">
        </div>
        {{name}}
    </div>
</template>


<style>
    /* h2 {
        margin: 10px 5px;
    } */
    .status {
        position: relative;
        padding: 10px;
        width: calc(100% -40px);
        border-radius: 20px;
        background-color: var(--background-color);
        color: var(--text-color);
        font-family: 'Major Mono Display', monospace;
        text-align: center;
        font-weight: 800;
    }
    .status .imgcontainer {
        padding: 10% 10% 0 10%;
    }
    .status img {
        width: 100%;
        border-radius: 100%;
    }

    .unkown { background-color: #5e5e5e }
    .free { background-color: #47f86d }
    .away { background-color: #f8ac47 }
    .busy { background-color: #f84d47 }
</style>