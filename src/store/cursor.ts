import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useCursorStore = defineStore('cursor', () => {

    const style = ref("none");

    const need_change = ref(true);

    function set_style(sty: string) {
        style.value = sty;
    }

    function set_change(need: boolean) {
        need_change.value = need;
    }

    return { style, need_change, set_style, set_change };
})