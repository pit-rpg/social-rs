export const ARRAY = {
    remove<T>(arr: T[], item: T): boolean {
        const index = arr.indexOf(item);

        if (index === -1) {
            return false;
        }

        arr.splice(index, 1);

        return true;
    },

    removeAll<T>(arr: T[], item: T): void {
        while (true) {
            const index = arr.indexOf(item);

            if (index === -1) {
                return;
            }

            arr.splice(index, 1);
        }
    },

    removeWith<T>(arr: T[], remover: (item: T, index: number, arr: T[]) => boolean | undefined): void {
        const index = arr.findIndex(remover);

        if (index !== -1) {
            arr.splice(index, 1);
        }
    },

    getIntersection<T>(arr1: T[], arr2: T[]): void {
        arr1.filter(item => arr2.includes(item));
    },

    getIntersectionWith<T>(arr1: T[], arr2: T[], func: (a: T, b: T) => boolean | undefined): T[] {
        return arr1.filter(a => arr2.some(b => func(a, b)));
    },

    unique<T>(arr: T[]): T[] {
        const res: T[] = [];

        for (const item of arr) {
            if (!res.includes(item)) {
                res.push(item);
            }
        }

        return res;
    },

    uniqueWith<T>(arr: T[], func: (item: T, res: T[], i: number, arr: T[]) => boolean | undefined): T[] {
        const res: T[] = [];

        arr.forEach((item, i) => {
            if (func(item, res, i, arr)) {
                res.push(item);
            }
        });

        return res;
    },

    uniqueBy<K extends string|number|symbol , T extends Record<K, any>>(arr: T[], key: K): T[] {
        return this.uniqueWith(arr, (item, res) => res.every(resItem => resItem[key] !== item[key]));
    },

    uniqueLastBy<K extends string|number|symbol , T extends Record<K, any>>(arr: T[], key: K): T[] {
        const deduplicator: Map<K, any> = new Map();

        arr.forEach(item => deduplicator.set(item[key], item));

        return arr.filter(item => deduplicator.get(item[key]) === item);
    },

    sortBy<K extends string|number|symbol , T extends Record<K, any>>(arr: T[], key: K, ascending: boolean = true): T[] {
        if (ascending) {
            return arr.sort((a, b) => a[key] < b[key] ? -1 : 1);
        }

        return arr.sort((a, b) => a[key] < b[key] ? 1 : -1);
    },

    removeNullableItems<T, K = NonNullable<T>>(arr: T[]): K[] {
        for (let i = 0; i < arr.length; i++) {
            if (!arr[i]) {
                arr.splice(i, 1);
                i--;
            }
        }

        return arr as any as K[];
    },

    updateBy<K extends string|number|symbol , T extends Record<K, any>>(arrOld: T[], arrNew: T[], key: K): T[] {
        arrOld.push(...arrNew);

        return this.uniqueLastBy(arrOld, key);
    }
} as const;