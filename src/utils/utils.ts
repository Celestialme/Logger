export function getClock(){
let time = new Date();
return `${zeroPad(time.getHours(),2)}:${zeroPad(time.getMinutes(),2)}:${zeroPad(time.getSeconds(),2)}`
}
 function zeroPad(nNum:number, nPad:number) {
    return ('' + (Math.pow(10, nPad) + nNum)).slice(1);
    };
export function hashTag(string:string,color:string) {
   var regexp = new RegExp('(#[^\\s]+)', 'gi');
   return string.replace(regexp, '<span style="color:'+color+'">$1</span>');
}