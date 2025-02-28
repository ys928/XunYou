import { defineStore } from 'pinia'
import { Ref, ref } from 'vue'
import { Bookmark, CataItem, Novel } from '../api/novel'
import { invoke } from '@tauri-apps/api/core'
import { useShowStore } from './show'
import * as dialog from "@tauri-apps/plugin-dialog"
import * as fs from "@tauri-apps/plugin-fs"

type Chapter = {
    /// 每章标题
    title: string,
    /// 每章内容，按行分割
    lines: Array<string>,
}

export const useNovelStore = defineStore('novel', () => {

    const show_store = useShowStore();

    // 当前是否已经打开小说
    const isopen = ref(false);

    // 当前小说名
    const name = ref('');

    // 小说所在路径
    const path = ref('');

    // 当前显示的章节索引
    const cur_ch_idx = ref(0);

    // 小说目录
    const cata = ref([]) as Ref<Array<CataItem>>;

    // 小说章节数量
    let num_chapters = 0;

    // 章节内行数索引
    const cur_line_idx = ref(0);

    //当前显示的章节内容
    const show_chapter = ref({ title: '', lines: [] }) as Ref<Chapter>;

    // 书签
    const bookmark = ref([]) as Ref<Array<Bookmark>>

    // 跳转函数
    let jump_novel_fun = (_chap: number, _line: number) => { };

    // 打开小说
    async function open(filepath: string) {
        let b = await fs.exists(filepath);
        if (!b) {
            await dialog.message('小说不存在！', { title: '打开失败', kind: 'warning' });
            return;
        }
        show_store.set_loading(true);
        // 保存路径
        path.value = filepath;

        // 保存文件名作为小说名
        let com_path = filepath.replaceAll('\\', '/');
        let idx = com_path.lastIndexOf('/');
        let filename = com_path.substring(idx + 1);
        name.value = filename.substring(0, filename.lastIndexOf('.'));
        console.log(name.value);

        if (filepath.endsWith('.txt')) {
            let ret = await Novel.open_txt(filepath);
            if (ret) {
                let re = await Novel.get_progress();
                show_chapter.value = await Novel.get_chapter(cur_ch_idx.value);
                show_store.set_loading(false);
                show_store.set_prompt(false);
                num_chapters = await Novel.get_num_chapters();
                cata.value = await Novel.get_cata();
                bookmark.value = await Novel.get_bookmark();
                call_jump_fun(re.chapter, re.line);
                isopen.value = true;
                return true;
            } else {
                show_store.set_loading(false);
                return false;
            }
        }
        show_store.set_loading(false);
        return false;
    }

    async function close() {
        isopen.value = false;
        cata.value = [];
        show_chapter.value.lines = [];
        show_chapter.value.title = '';
    }

    async function add_bookmark(mark: Bookmark) {
        let m = { ...mark };
        bookmark.value.push(m)
        console.log(bookmark.value);

        await Novel.add_bookmark(m);
    }

    async function del_bookmark(index: number) {
        await invoke("novel_del_bookmark", {
            id: bookmark.value[index].id,
        });
        bookmark.value.splice(index, 1);
    }

    async function next_chapter() {
        if (cur_ch_idx.value >= num_chapters - 1) {
            return false;
        }
        show_chapter.value = await Novel.get_chapter(cur_ch_idx.value + 1);
        cur_ch_idx.value += 1;
        save_record();
        return true;
    }

    async function prev_chapter() {
        if (cur_ch_idx.value > 0) {
            show_chapter.value = await Novel.get_chapter(cur_ch_idx.value - 1);
            cur_ch_idx.value -= 1;
            save_record();
            return true;
        }
        return false;
    }

    async function set_show_chapter(chap: number) {
        show_chapter.value = await Novel.get_chapter(chap);
        cur_ch_idx.value = chap;
        save_record();
    }

    async function set_jump_fun(fun_jump: (chap: number, line: number) => Promise<void>) {
        jump_novel_fun = fun_jump;
    }

    async function call_jump_fun(chap: number, line: number) {
        jump_novel_fun(chap, line);
        save_record();
    }

    async function set_line(line: number) {
        cur_line_idx.value = line;
        await save_record();
    }

    async function save_record() {
        await Novel.save_record({
            chapter: cur_ch_idx.value,
            line: cur_line_idx.value,
        });
    }

    async function jump_to_bookmark(index: number) {
        call_jump_fun(bookmark.value[index].chapter, bookmark.value[index].line);
    }

    return { name, path, cur_ch_idx, cur_line_idx, show_chapter, bookmark, isopen, cata, set_line, open, close, jump_to_bookmark, add_bookmark, del_bookmark, next_chapter, prev_chapter, set_show_chapter, set_jump_fun, call_jump_fun };
})