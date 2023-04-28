<script setup>
  import Clocks from '/js/components/clocks.vue'
  import Temps from '/js/components/temps.vue'
  import Servers from '/js/components/servers.vue'
  import Image from '/js/components/image.vue'
  import Reminders from '/js/components/reminders.vue'
  import Status from '/js/components/status.vue'

</script>`;

<template>
    <div class="body">
        
        <div class="container">
            <main>
                <!-- COLUMN 1 -->
                <div class="col1width2 row1height5">
                    <Servers />
                </div>
                <!-- COLUMN 3 -->
                <div class="col3width2 row1height2">
                    <Clocks />
                </div>
                <div class="col3width2 row3height2">
                    <Temps />
                </div>
                <div class="col3width1 row5height1">
                    <Image src="/img/custom/dance-dance-moves.gif" />
                </div>
                <div class="col4width1 row5height1">
                    <Image src="/img/custom/monkey-funny-monke.gif" />
                </div>
                <!-- COLUMN 5 -->
                <div class="col5width1 row1height1">
                    <Status name="jordan" image="jordan.jpg" status="busy"/>
                </div>
                <div class="col6width1 row1height1">
                    <Status name="lachlan" image="lachlan.jpg" status="free"/>
                </div>
                <div class="col5width2 row2height4">
                    <Reminders />
                </div>
            </main>
        </div>
    </div>
</template>



<style>
    .container {
        position: relative;
        overflow: hidden;
        height: 100%;
    }
    .container main {
        display: grid;
        height: calc(100vh - 100px);
        padding: 0px 40px 40px 40px;
        grid-template-columns: repeat(10, minmax(0, 1fr));
        grid-template-rows: repeat(5, minmax(0, 1fr));
        gap: 20px;
    }

    main>div {
        overflow-y: scroll;
    }

    main>*::-webkit-scrollbar {
        display: none;
    }

    .col1width1 { grid-column-start: 1; grid-column-end: 2 }
    .col2width1 { grid-column-start: 2; grid-column-end: 3 }
    .col3width1 { grid-column-start: 3; grid-column-end: 4 }
    .col4width1 { grid-column-start: 4; grid-column-end: 5 }
    .col5width1 { grid-column-start: 5; grid-column-end: 6 }
    .col6width1 { grid-column-start: 6; grid-column-end: 7 }
    .col7width1 { grid-column-start: 7; grid-column-end: 8 }
    .col8width1 { grid-column-start: 8; grid-column-end: 9 }
    .col9width1 { grid-column-start: 9; grid-column-end: 10 }
    .col10width1 { grid-column-start: 10; grid-column-end: 11 }
    .col1width2 { grid-column-start: 1; grid-column-end: 3 }
    .col2width2 { grid-column-start: 2; grid-column-end: 4 }
    .col3width2 { grid-column-start: 3; grid-column-end: 5 }
    .col4width2 { grid-column-start: 4; grid-column-end: 6 }
    .col5width2 { grid-column-start: 5; grid-column-end: 7 }
    .col6width2 { grid-column-start: 6; grid-column-end: 8 }
    .col7width2 { grid-column-start: 7; grid-column-end: 9 }
    .col8width2 { grid-column-start: 8; grid-column-end: 10 }
    .col9width2 { grid-column-start: 9; grid-column-end: 11 }
    .col1width3 { grid-column-start: 1; grid-column-end: 4 }
    .col2width3 { grid-column-start: 2; grid-column-end: 5 }
    .col3width3 { grid-column-start: 3; grid-column-end: 6 }
    .col4width3 { grid-column-start: 4; grid-column-end: 7 }
    .col5width3 { grid-column-start: 5; grid-column-end: 8 }
    .col6width3 { grid-column-start: 6; grid-column-end: 9 }
    .col7width3 { grid-column-start: 7; grid-column-end: 10 }
    .col8width3 { grid-column-start: 8; grid-column-end: 11 }
    .col1width4 { grid-column-start: 1; grid-column-end: 5 }
    .col2width4 { grid-column-start: 2; grid-column-end: 6 }
    .col3width4 { grid-column-start: 3; grid-column-end: 7 }
    .col4width4 { grid-column-start: 4; grid-column-end: 8 }
    .col5width4 { grid-column-start: 5; grid-column-end: 9 }
    .col6width4 { grid-column-start: 6; grid-column-end: 10 }
    .col7width4 { grid-column-start: 7; grid-column-end: 11 }
    .col1width5 { grid-column-start: 1; grid-column-end: 6 }
    .col2width5 { grid-column-start: 2; grid-column-end: 7 }
    .col3width5 { grid-column-start: 3; grid-column-end: 8 }
    .col4width5 { grid-column-start: 4; grid-column-end: 9 }
    .col5width5 { grid-column-start: 5; grid-column-end: 10 }
    .col6width5 { grid-column-start: 6; grid-column-end: 11 }
    .col1width6 { grid-column-start: 1; grid-column-end: 7 }
    .col2width6 { grid-column-start: 2; grid-column-end: 8 }
    .col3width6 { grid-column-start: 3; grid-column-end: 9 }
    .col4width6 { grid-column-start: 4; grid-column-end: 10 }
    .col5width6 { grid-column-start: 5; grid-column-end: 11 }
    .row1height1 { grid-row-start: 1; grid-row-end: 2 }
    .row2height1 { grid-row-start: 2; grid-row-end: 3 }
    .row3height1 { grid-row-start: 3; grid-row-end: 4 }
    .row4height1 { grid-row-start: 4; grid-row-end: 5 }
    .row5height1 { grid-row-start: 5; grid-row-end: 6 }
    .row1height2 { grid-row-start: 1; grid-row-end: 3 }
    .row2height2 { grid-row-start: 2; grid-row-end: 4 }
    .row3height2 { grid-row-start: 3; grid-row-end: 5 }
    .row4height2 { grid-row-start: 4; grid-row-end: 6 }
    .row1height3 { grid-row-start: 1; grid-row-end: 4 }
    .row2height3 { grid-row-start: 2; grid-row-end: 5 }
    .row3height3 { grid-row-start: 3; grid-row-end: 6 }
    .row1height4 { grid-row-start: 1; grid-row-end: 5 }
    .row2height4 { grid-row-start: 2; grid-row-end: 6 }
    .row1height5 { grid-row-start: 1; grid-row-end: 6 }
</style>