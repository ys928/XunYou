import { invoke } from "@tauri-apps/api/core";
import { readDir } from "@tauri-apps/plugin-fs";
import * as path from '@tauri-apps/api/path';
export type NovelInfo = {
    name: string,
    path: string,
}


export class FS {
    static async read_dir(p: string): Promise<NovelInfo[]> {
        let files = await readDir(p);
        let names = [] as Array<NovelInfo>;
        for (let f of files) {
            names.push({
                name: f.name as string,
                path: await path.join(p, f.name)
            })
        }
        return names;
    }

    static async set_root_path(p: string) {
        invoke("cfg_set_novel_folder", { folder: p });
    }

    static async get_root_path(): Promise<string> {
        return await invoke<string>("cfg_get_novel_folder", {});
    }
}