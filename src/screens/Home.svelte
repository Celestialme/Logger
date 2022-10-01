<script lang="ts">
    let left_panel:HTMLDivElement;
    let top_panel:HTMLDivElement;
    let logs_container:HTMLDivElement;
    let inputValue:string="";
    let time:string=getClock();
    let timer:NodeJS.Timer;
    let showRightPanel = true;
    import Resizer from "@src/components/Resizer.svelte"
    import Logs from  "@src/components/Logs.svelte"
    import Arrow from "@src/components/Arrow.svelte";
    import { getClock } from "@src/utils/utils";
    import { onDestroy, onMount, tick } from "svelte";
    import {logs} from "@src/store/store"
    import { invoke } from '@tauri-apps/api/tauri'
    function addLog(){
        let log = {time:getClock(),log:inputValue}
        $logs.push(log);
        $logs = $logs
        invoke("write_log",{log:`${log.time} > ${log.log}`});
        inputValue="";
        tick().then(()=>logs_container.scrollTop = logs_container.scrollHeight)
    }
    onMount(()=>{
    timer = setInterval(()=>time = getClock(),1000)
    });
    onDestroy(()=>{
        clearInterval(timer)
    })
</script>

<div class="body">
    <div class="top-panel" bind:this={top_panel}>

        <div class="left-panel" bind:this={left_panel} >
        <button on:click={(e)=>showRightPanel=!showRightPanel}>Show/Hide</button>
        </div>
        {#if showRightPanel}
        <Resizer class="vertical" {left_panel} {top_panel}/>
        <div class="right-panel" >
        
        </div>
        {/if}

    </div>

    <Resizer class="horizontal" {left_panel} {top_panel}/>

    <div class="bottom-panel" >
        <div class="logs-container" bind:this={logs_container}>
            <Logs />
        </div>

        <div class="input-container">
          <span class="clock">{time}</span> <Arrow angle={55}/> <input bind:value={inputValue} type="text" on:keydown={({key})=>{key=="Enter"&&addLog()}}>
        </div>

    </div>


</div>


<style>
    @import url('https://fonts.googleapis.com/css2?family=Rajdhani&display=swap');
    @import url('https://fonts.googleapis.com/css2?family=Gemunu+Libre&display=swap');

    button{
        padding:20px;
        border-radius: 12px;
    }
    .body{
       
        display:flex;
        flex-direction: column;
        width:100%;
        height: 100vh;
    }
  
    .top-panel{
        height: 70vh;
        display: flex;

        
    }
   
    .left-panel{
        min-width: 20vw;
        width:40vw;
        background: #484747;
      
    }
    .left-panel:only-child{
        width:100vw !important;
    }
    .right-panel{
        flex-grow: 1;
   
    }
    
    .bottom-panel{
        display: flex;
        flex-basis: 0px;
        
        flex-direction: column;
        flex-grow:1;
        border:5px solid #5C5C5C;
        border-top: none;
 
    }
    .logs-container{
        overflow-y: auto;
        flex-basis: 0px;
        flex-grow: 1;
        border-bottom:3px solid #5C5C5C;
        padding-bottom:2px;
        
    }
    .input-container{
        display:flex;
        gap:5px;
        align-items: center;
        padding:5px
    }
    .input-container input{
        font-family: 'Rajdhani', sans-serif;
        flex-grow: 1;
        margin-left:10px;
        background:transparent;
        font-size: 20px;
        padding:2px;
        outline:none;
        border:none;
        color:white;
    }
    .clock{
        width:70px;
        margin-top: 2px;
        margin-left:10px;
        font-family: 'Gemunu Libre', sans-serif;
        font-size: 22px;
        color:white;
    }
   
</style>