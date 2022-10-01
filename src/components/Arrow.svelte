<script lang="ts">
export let angle:Number=60;
export let size:Number=14;
export let thickness:Number=2;
export let color:string="#A5A5A5";
let arrow:HTMLDivElement;
$:{ if(arrow){
    arrow.style.setProperty("--size",size+"px")
    arrow.style.setProperty("--angle",angle+"deg")
    arrow.style.setProperty("--color",color)
    arrow.style.setProperty("--thickness",thickness+"px")
}
    }
</script>
<div class="arrow" bind:this={arrow}></div>
<style>
     .arrow::before,.arrow::after{
        position: absolute;
    display: inline-block;
    content: "";
    background-color: var(--color);
    width: var(--size);
    height:var(--thickness);
    top: 50%;
    
    
    }
    .arrow{
        display: inline-block;
        position: relative;
        width:var(--size);
        height:var(--size);
     
    }
    .arrow::before{
        transform: rotate(calc( var(--angle) * -1) );
        transform-origin: right top;
        border-bottom-right-radius: 5px;
        border-top-right-radius: 5px;
      
    }
    .arrow::after{
        transform: rotate(var(--angle));
        transform-origin: right bottom;
        border-top-right-radius: 5px;
        border-bottom-right-radius: 5px;
       
    }
</style>