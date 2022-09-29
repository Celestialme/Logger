<script lang="ts">
    import Arrow from "./Arrow.svelte";
    import {logs} from "@src/store/store"
    import { invoke } from '@tauri-apps/api/tauri'
    let data = ""
  
    invoke("read_logs").then((_logs)=>{data = _logs as string,console.log(_logs)
    
    
        
    for (let log of data.matchAll(/(\d+:\d+:\d+)\s+>\s+(.*)/g)){
        $logs.push({time:log[1],log:log[2]})
    }
    $logs = $logs
    });

</script>

{#each $logs as log}
<div> 
    <span class="clock">{log.time}</span>
    <Arrow angle={50} size={16} color="#F9BD23"/>
    <span class="log">{log.log}</span>
</div>
{/each}
<style>
    div{
        padding:4px;
    }
    span.clock{
        margin-left: 15px;
        margin-right: 10px;
        display: inline-block;
        width:80px;
        font-family: 'Gemunu Libre', sans-serif;
        color:#D89696;
        font-size: 25px;
    }
    span.log{
        margin-left: 15px;
        font-family: 'Gemunu Libre', sans-serif;
        color:#D9D9D9;
        font-size: 25px;
    }
</style>