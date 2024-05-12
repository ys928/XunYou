import { invoke } from "@tauri-apps/api";
type Chapter = {
    /// 每章标题
    title: string,
    /// 每章内容，按行分割
    lines: Array<string>,
}
export class Novel {
    static async open_txt(filepath: string): Promise<boolean> {
        return await invoke<boolean>("novel_open_txt", { filepath: filepath });
    }

    static async get_chapter(idx: number) {
        return await invoke<Chapter>("novel_get_chapter", { idx: idx });
    }
}