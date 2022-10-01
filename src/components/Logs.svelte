<script lang="ts">
    import Arrow from "./Arrow.svelte";
    import {logs} from "@src/store/store"
    import { invoke } from '@tauri-apps/api/tauri'
    let data = ""
  
    invoke("read_logs").then((_logs)=>{data = _logs as string
    
    
        
    for (let log of data.matchAll(/(\d+:\d+:\d+)\s+>\s+(.*)/g)){
        $logs.push({time:log[1],log:log[2]})
    }
    $logs = $logs
    });

</script>
<div class="padding">
    {#each $logs as log}
    <div> 
        <span class="clock">{log.time}</span>
        <Arrow angle={30} size={8} thickness={1} color="#F9BD23"/>
        <span class="log">{log.log}</span>
    </div>
    {/each}
</div>
<style>
   
    span.clock{
        margin-left: 15px;
        display: inline-block;
        width:50px;
        font-family: 'Gemunu Libre', sans-serif;
        color:#D89696;
        font-size: 14px;
    }
    span.log{
        margin-left: 5px;
        font-family: 'Gemunu Libre', sans-serif;
        color:#D9D9D9;
        font-size: 14px;
       
    }
    .padding{
        height: 100%;
    overflow-y: auto;
    line-height: 15px;
    }
</style>