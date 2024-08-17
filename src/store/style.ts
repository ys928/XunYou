import { invoke } from '@tauri-apps/api';
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useStyleStore = defineStore('style', () => {
    //字体大小
    const font_size = ref(16);
    //字体粗细
    const font_weight = ref(400);
    //字体
    const font_family = ref("宋体");
    //行高
    const line_height = ref(16);
    //渲染样式
    const style = ref({
        fontSize: font_size.value + 'px',
        lineHeight: line_height.value / 10 + 'em',
        backgroundSize: '15px ' + line_height.value / 10 + 'em',
        fontFamily: font_family.value,
        fontWeight: font_weight.value,
    });

    function set(fsize: number, fweight: number, ffamily: string, lheight: number) {
        font_size.value = fsize;
        font_weight.value = fweight;
        font_family.value = ffamily;
        line_height.value = lheight;
        render();
    }

    function set_font_size(size: number) {
        font_size.value = size;
        render();
    }

    function set_font_family(family: string) {
        font_family.value = family;
        render();
    }

    function set_font_weight(weight: number) {
        font_weight.value = weight;
        render();
    }

    function set_line_height(height: number) {
        line_height.value = height;
        render();
    }

    function render() {
        style.value = {
            fontSize: font_size.value + 'px',
            lineHeight: line_height.value / 10 + 'em',
            backgroundSize: '15px ' + line_height.value / 10 + 'em',
            fontFamily: font_family.value,
            fontWeight: font_weight.value,
        };
    }

    function save() {
        invoke("cfg_app_set_style", {
            style: {
                font_size: font_size.value,
                font_weight: font_weight.value,
                line_height: line_height.value,
                font_family: font_family.value
            }
        });
    }

    return { font_family, font_size, font_weight, line_height, style, set, set_font_size, set_font_family, set_font_weight, set_line_height, save }
})