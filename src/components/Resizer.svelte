<script lang="ts">
    export let left_panel:HTMLDivElement;
    export let top_panel:HTMLDivElement;
  function pointerevent(event:PointerEvent,callBack:(event:PointerEvent)=>void) {
        let el = (event.target as HTMLDivElement);
        if(event.type=="pointerup"){
            el.onpointermove=null;
            return   
        }
        el.onpointermove = callBack 

        el.setPointerCapture(event.pointerId) 
 
        
        
    }
    function horizontal_resize(event:PointerEvent) {
        event.clientX && (left_panel.style.width = event.clientX+"px")
    }
    function vertical_resize(event:PointerEvent) {
        event.clientY && (top_panel.style.height = event.clientY+"px")
    }

</script>

<div class={$$props.class} draggable="false" on:pointerup={(e)=>pointerevent(e,$$props.class.includes("vertical")?horizontal_resize:vertical_resize)} on:pointerdown={(e)=>pointerevent(e,$$props.class.includes("vertical")?horizontal_resize:vertical_resize)}  ></div>
<style>
    .vertical{
        cursor:e-resize;
        width:5px;
        background-color:  #5C5C5C;
    }
    .horizontal{
        cursor:n-resize;
        height:5px;
        background-color:  #5C5C5C;
    }
</style>