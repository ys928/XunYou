import { invoke } from "@tauri-apps/api";
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

export class Novel {
    static async open_txt(filepath: string): Promise<boolean> {
        return await invoke<boolean>("novel_open_txt", { filepath: filepath });
    }

    static async get_chapter(idx: number) {
        return await invoke<Chapter>("novel_get_chapter", { idx: idx });
    }

    static async get_record() {
        return await invoke<Record>("novel_get_record");
    }

    static async get_num_chapters() {
        return await invoke<number>("novel_get_num_chapters");
    }
}