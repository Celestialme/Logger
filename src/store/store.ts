import {writable, type Writable} from "svelte/store";

export let logs:Writable<Array<any>> = writable([]);