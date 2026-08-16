import { defineStore } from 'pinia'
import { reactive, Ref, ref, nextTick } from 'vue'
import { Bookmark, CataItem, EncodingPreview, encoding_display_name, Novel } from '../api/novel'
import { invoke } from '@tauri-apps/api/core'
import { useShowStore } from './show'
import { ElMessage } from 'element-plus'
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

    // 当前文本编码
    const encoding = ref('UTF-8');

    const encoding_dialog = reactive({
        visible: false,
        filepath: '',
        previews: [] as EncodingPreview[],
    });
    let applying_encoding = false;

    // 当前显示的章节索引
    const cur_ch_idx = ref(0);

    // 小说目录
    const cata = ref([]) as Ref<Array<CataItem>>;

    // 小说章节数量
    let num_chapters = 0;

    // 各章段落行数
    const chapter_line_counts = ref<number[]>([]);

    // 章节内行数索引
    const cur_line_idx = ref(0);

    // 分页信息（由阅读器根据可视行数计算）
    const chapter_page = ref(0);
    const chapter_page_count = ref(0);
    const remaining_pages = ref(0);
    const book_page = ref(0);
    const book_pages = ref(0);

    //当前显示的章节内容
    const show_chapter = ref({ title: '', lines: [] }) as Ref<Chapter>;

    // 书签
    const bookmark = ref([]) as Ref<Array<Bookmark>>

    // 跳转函数
    let jump_novel_fun: (chap: number, line: number) => void | Promise<void> = () => { };

    function show_encoding_dialog(filepath: string, previews: EncodingPreview[]) {
        encoding_dialog.filepath = filepath;
        encoding_dialog.previews = previews;
        encoding_dialog.visible = true;
    }

    function close_encoding_dialog() {
        encoding_dialog.visible = false;
        if (applying_encoding) {
            return;
        }
        if (!isopen.value) {
            show_store.set_prompt(true);
        }
    }

    async function apply_encoding(enc: string) {
        const filepath = encoding_dialog.filepath;
        applying_encoding = true;
        encoding_dialog.visible = false;
        try {
            if (!filepath) {
                return false;
            }
            return await open(filepath, enc);
        } finally {
            applying_encoding = false;
        }
    }

    async function show_encoding_picker() {
        if (!path.value) {
            return;
        }
        show_store.set_loading(true);
        try {
            const previews = await Novel.preview_encodings(path.value);
            show_encoding_dialog(path.value, previews);
        } catch (e) {
            await dialog.message(String(e), { title: '无法预览编码', kind: 'warning' });
        }
        show_store.set_loading(false);
    }

    async function finish_open() {
        const re = await Novel.get_progress();
        cur_ch_idx.value = re.chapter ?? 0;
        cur_line_idx.value = re.line ?? 0;
        try {
            show_chapter.value = await Novel.get_chapter(cur_ch_idx.value);
        } catch {
            cur_ch_idx.value = 0;
            cur_line_idx.value = 0;
            show_chapter.value = await Novel.get_chapter(0);
        }
        show_store.set_prompt(false);
        num_chapters = await Novel.get_num_chapters();
        cata.value = await Novel.get_cata();
        bookmark.value = await Novel.get_bookmark();
        try {
            chapter_line_counts.value = await Novel.get_chapter_line_counts();
        } catch {
            chapter_line_counts.value = [];
        }
        isopen.value = true;
        await nextTick();
        await call_jump_fun(cur_ch_idx.value, cur_line_idx.value);
        show_store.set_loading(false);
        return true;
    }

    // 打开小说
    async function open(filepath: string, forced_encoding?: string) {
        let b = await fs.exists(filepath);
        if (!b) {
            await dialog.message('小说不存在！', { title: '打开失败', kind: 'warning' });
            return false;
        }
        show_store.set_loading(true);

        if (filepath.endsWith('.txt')) {
            try {
                const ret = await Novel.open_txt(filepath, forced_encoding);
                if (ret.status === 'need_pick') {
                    show_store.set_loading(false);
                    show_encoding_dialog(filepath, ret.previews);
                    return false;
                }
                path.value = filepath;
                let com_path = filepath.replaceAll('\\', '/');
                let idx = com_path.lastIndexOf('/');
                let filename = com_path.substring(idx + 1);
                name.value = filename.substring(0, filename.lastIndexOf('.'));
                encoding.value = ret.encoding || 'UTF-8';
                const ok = await finish_open();
                if (ok && ret.auto_detected && encoding.value !== 'UTF-8') {
                    ElMessage.success(`已按 ${encoding_display_name(encoding.value)} 打开`);
                }
                return ok;
            } catch (e) {
                show_store.set_loading(false);
                if (!isopen.value) {
                    show_store.set_prompt(true);
                }
                await dialog.message(String(e), { title: '打开失败', kind: 'error' });
                return false;
            }
        }
        show_store.set_loading(false);
        return false;
    }

    async function close() {
        isopen.value = false;
        encoding.value = 'UTF-8';
        cata.value = [];
        chapter_line_counts.value = [];
        show_chapter.value.lines = [];
        show_chapter.value.title = '';
        chapter_page.value = 0;
        chapter_page_count.value = 0;
        remaining_pages.value = 0;
        book_page.value = 0;
        book_pages.value = 0;
    }

    function set_pager_state(state: {
        chapterPage: number
        chapterPageCount: number
        remainingPages: number
        bookPage: number
        bookPages: number
    }) {
        chapter_page.value = state.chapterPage;
        chapter_page_count.value = state.chapterPageCount;
        remaining_pages.value = state.remainingPages;
        book_page.value = state.bookPage;
        book_pages.value = state.bookPages;
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
        await jump_novel_fun(chap, line);
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

    return { name, path, encoding, encoding_dialog, cur_ch_idx, cur_line_idx, show_chapter, bookmark, isopen, cata, chapter_line_counts, chapter_page, chapter_page_count, remaining_pages, book_page, book_pages, set_pager_state, set_line, open, close, jump_to_bookmark, add_bookmark, del_bookmark, next_chapter, prev_chapter, set_show_chapter, set_jump_fun, call_jump_fun, show_encoding_picker, apply_encoding, close_encoding_dialog };
})