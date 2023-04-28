<script>
    export default {
        props: ['src']
    }
</script>
<template>
    <div class="img" :style="{'background-image': 'url(' + src + ')'}">
    </div>
    
</template>


<style>
    .img {
        -ms-grid-column-span: 2;
        position: relative;
        padding: 20px;
        width: calc(100% - 40px);
        height: calc(100% - 40px);
        border-radius: 20px;
        background-position: center center;
        background-repeat: no-repeat;
        -webkit-background-size: cover;
        -moz-background-size: cover;
        -o-background-size: cover;
        background-size: cover;
    }
</style>