<script>
    export default {
        data() {
            return {
                interval: null,
                clocks: [
                    {hour: "00", minute: "00", ampm: "AM", state: "qld"},
                    {hour: "00", minute: "00", ampm: "AM", state: "vic"},
                    {hour: "00", minute: "00", ampm: "AM", state: "nsw"},
                    {hour: "00", minute: "00", ampm: "AM", state: "sa"},
                    {hour: "00", minute: "00", ampm: "AM", state: "wa"},
                ]
            };
        },
        created() {
            this.interval = setInterval(() => {
                let qld = new Intl.DateTimeFormat([], {timeZone: 'Australia/Brisbane', hour: '2-digit', minute: '2-digit'}).format().split(/[:\s]+/);
                let vic = new Intl.DateTimeFormat([], {timeZone: 'Australia/Melbourne', hour: '2-digit', minute: '2-digit'}).format().split(/[:\s]+/);
                let nsw = new Intl.DateTimeFormat([], {timeZone: 'Australia/Sydney', hour: '2-digit', minute: '2-digit'}).format().split(/[:\s]+/);
                let sa = new Intl.DateTimeFormat([], {timeZone: 'Australia/Adelaide', hour: '2-digit', minute: '2-digit'}).format().split(/[:\s]+/);
                let wa = new Intl.DateTimeFormat([], {timeZone: 'Australia/Perth', hour: '2-digit', minute: '2-digit'}).format().split(/[:\s]+/);
                this.clocks = [
                    {hour: qld[0], minute: qld[1], ampm: qld[2].toUpperCase(), state: "qld"},
                    {hour: vic[0], minute: vic[1], ampm: vic[2].toUpperCase(), state: "vic"},
                    {hour: nsw[0], minute: nsw[1], ampm: nsw[2].toUpperCase(), state: "nsw"},
                    {hour: sa[0], minute: sa[1], ampm: sa[2].toUpperCase(), state: "sa"},
                    {hour: wa[0], minute: wa[1], ampm: wa[2].toUpperCase(), state: "wa"}
                ];
            })
        }
    };
</script>
<template>
    <div class="clocks">
        <h2>time</h2>
        <div class="clock" v-for="{hour, minute, ampm, state} in clocks">
            <div class="time">{{ hour }}<span>:</span>{{ minute }}{{ ampm }}</div>
            <div class="state">{{ state }}</div>
        </div>
    </div>    
</template>

<style>
    h2 {
        margin: 10px 5px;
    }
    .clocks {
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
    .clock {
        display: flex;
        justify-content: space-between;
        border-bottom: 3px solid var(--divider-color);
    }
    .clock .time {
        letter-spacing: -4px;
        font-size: 48px;
    }

    .clock .time span {
        letter-spacing: -10px;
        margin-left: -5px;
    }
    .clock .state {
        width: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 20px;
    }
</style>