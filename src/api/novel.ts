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

export type EncodingPreview = {
    encoding: string,
    display_name: string,
    preview: string,
    recommended: boolean,
}

export type OpenTxtResult = {
    status: "ok" | "need_pick",
    encoding: string | null,
    auto_detected: boolean,
    previews: EncodingPreview[],
}

export function encoding_display_name(encoding: string): string {
    switch (encoding) {
        case "GB18030": return "GB18030（简体中文）";
        case "Big5": return "Big5（繁体中文）";
        case "UTF-16LE": return "UTF-16 LE";
        case "UTF-16BE": return "UTF-16 BE";
        case "windows-1252": return "Windows-1252";
        default: return encoding || "UTF-8";
    }
}

export class Novel {
    static async open_txt(filepath: string, encoding?: string | null): Promise<OpenTxtResult> {
        return await invoke<OpenTxtResult>("novel_open_txt", {
            filepath,
            encoding: encoding ?? null,
        });
    }

    static async preview_encodings(filepath: string): Promise<EncodingPreview[]> {
        return await invoke<EncodingPreview[]>("novel_preview_encodings", { filepath });
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

    static async get_chapter_line_counts() {
        return await invoke<Array<number>>("novel_get_chapter_line_counts");
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
