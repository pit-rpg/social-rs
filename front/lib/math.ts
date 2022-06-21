export const MATH = {
    trim(num: number, min: number, max: number) {
        return Math.min(Math.max(0, num), max);
    },
} as const;