import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia'
import { Ref, ref } from 'vue'

export const useStyleStore = defineStore('style', () => {
    //字体大小
    const font_size = ref(16);
    //字体粗细
    const font_weight = ref(400);
    //字体
    const font_family = ref("宋体");
    //行高
    const line_height = ref(16);

    function set(fsize: number, fweight: number, ffamily: string, lheight: number) {
        font_size.value = fsize;
        font_weight.value = fweight;
        font_family.value = ffamily;
        line_height.value = lheight;
    }

    function set_font_size(size: number) {
        font_size.value = size;
    }

    function set_font_family(family: string) {
        font_family.value = family;
    }

    function set_font_weight(weight: number) {
        font_weight.value = weight;
    }

    function set_line_height(height: number) {
        line_height.value = height;
    }

    function save() {
        invoke("set_setting", {
            fs: font_size.value,
            fw: font_weight.value,
            lh: line_height.value,
            ff: font_family.value
        });
    }

    return { font_family, font_size, font_weight, line_height, set, set_font_size, set_font_family, set_font_weight, set_line_height, save }
})