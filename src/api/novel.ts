import { invoke } from "@tauri-apps/api/core";
type Chapter = {
    /// 每章标题
    title: string,
    /// 每章内容，按行分割
    lines: Array<string>,
}

type Record = {
    /// 每章标题
    chapter: number,
    /// 每章内容，按行分割
    line: number,
}

export type Bookmark = {
    id: string, //识别该书签的唯一id
    label: string, //该标签的额外标注信息
    chapter: number, //所属章节
    line: number, //所属行
    datetime: string, //创建日期
    content: string, //简短文章内容
}

export type CataItem = {
    title: string,
    idx: number
}

export class Novel {
    static async open_txt(filepath: string): Promise<boolean> {
        return await invoke<boolean>("novel_open_txt", { filepath: filepath });
    }

    static async get_chapter(idx: number) {
        return await invoke<Chapter>("novel_get_chapter", { idx: idx });
    }

    static async get_progress() {
        return await invoke<Record>("novel_get_progress");
    }

    static async get_num_chapters() {
        return await invoke<number>("novel_get_num_chapters");
    }

    static async get_cata() {
        return await invoke<Array<CataItem>>("novel_get_cata");
    }

    static async get_bookmark() {
        return await invoke<Array<Bookmark>>("novel_get_bookmark");
    }

    static async add_bookmark(mark: Bookmark) {
        return await invoke<boolean>("novel_add_bookmark", { mark: mark });
    }

    static async save_record(record: Record) {
        return await invoke<boolean>("novel_set_progress", { record: record });
    }
}