<script lang="ts">
    import Arrow from "./Arrow.svelte";
    import {logs} from "@src/store/store"
    import { invoke } from '@tauri-apps/api/tauri'
    import {hashTag} from "@src/utils/utils"
    let data = ""
  
    invoke("read_logs").then((_logs)=>{data = _logs as string
    
    
        
    for (let log of data.matchAll(/(\d+:\d+:\d+)\s+>\s+(.*)/g)){
        $logs.push({time:log[1],log:log[2]})
    }
    $logs = $logs
    });

</script>

    {#each $logs as log}
    <div> 
        <span class="clock">{log.time}</span>
        <Arrow angle={30} size={8} thickness={1} color="#F9BD23"/>
        <span class="log">{@html hashTag(log.log,"#9F9F9F")}</span>
    </div>
    {/each}

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
  
</style>