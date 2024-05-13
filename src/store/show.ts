import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useShowStore = defineStore('show', () => {
    // 中心提示信息
    const show_prompt = ref(true);

    // 中心加载动图
    const show_loading = ref(false);

    function set_prompt(show: boolean) {
        show_prompt.value = show;
    }

    function set_loading(show: boolean) {
        show_loading.value = show;
    }

    return { show_loading, show_prompt, set_loading, set_prompt };
})