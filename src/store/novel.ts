import { defineStore } from 'pinia'
import { Ref, ref } from 'vue'
import { Novel } from '../api/novel'
import { dialog, fs, invoke } from '@tauri-apps/api'
import { useShowStore } from './show'


type Chapter = {
    /// 每章标题
    title: string,
    /// 每章内容，按行分割
    lines: Array<string>,
}

type Bookmark = {
    id: string, //识别该书签的唯一id
    label: string, //该标签的额外标注信息
    chapter: Number, //所属章节
    line: Number, //所属行
    datetime: string, //创建日期
    content: string, //简短文章内容
}

export const useNovelStore = defineStore('novel', () => {

    const show_store = useShowStore();

    // 当前小说名
    const name = ref('');

    // 小说所在路径
    const path = ref('');

    // 当前显示的章节索引
    const cur_ch_idx = ref(0);

    // 章节内行数索引
    const cur_line_idx = ref(0);

    //当前显示的章节内容
    const show_chapter = ref({ title: '', lines: [] }) as Ref<Chapter>;

    // 书签
    const bookmark = ref([]) as Ref<Array<Bookmark>>

    // 打开小说
    async function open(filepath: string) {
        let b = await fs.exists(filepath);
        if (!b) {
            await dialog.message('小说不存在！', { title: '打开失败', type: 'warning' });
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
                let re = await Novel.get_record();
                cur_ch_idx.value = re.chapter;
                cur_line_idx.value = re.line;
                show_chapter.value = await Novel.get_chapter(cur_ch_idx.value);
                show_store.set_loading(false);
                show_store.set_prompt(false);
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

    }

    async function add_bookmark(mark: Bookmark) {
        bookmark.value.push(mark)
        await invoke('add_bookmark', {
            path: path.value,
            mark: bookmark.value
        });
    }

    async function next_chapter() {
        show_chapter.value = await Novel.get_chapter(cur_ch_idx.value + 1);
        cur_ch_idx.value += 1;
    }

    async function prev_chapter() {
        if (cur_ch_idx.value > 0) {
            show_chapter.value = await Novel.get_chapter(cur_ch_idx.value - 1);
            cur_ch_idx.value -= 1;
        }
    }

    async function set_show_chapter(chap: number) {
        show_chapter.value = await Novel.get_chapter(chap);
        cur_ch_idx.value = chap;
    }

    return { name, path, cur_ch_idx, cur_line_idx, show_chapter, bookmark, open, close, add_bookmark, next_chapter, prev_chapter, set_show_chapter };
})