import fs from 'fs';

export const PATH = {
    async ensureDirExist(path: string) {
        const stat = await fs.promises.lstat(path);

        if (stat.isDirectory()) return;

        return fs.promises.mkdir(path, {recursive: true})
    },
} as const;