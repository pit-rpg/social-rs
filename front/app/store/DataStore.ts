import {EventEmitter} from 'events';

export enum DataStoreEvents {
    CHANGE = 'CHANGE',
};

export type Latiner<T extends Record<string, any>, K extends keyof T> = (eventKey: K, val: T[K], oldVal: T[K]) => void;

export class DataStore<T extends Record<string, any>> extends EventEmitter {

    constructor() {
        super();

        this.setMaxListeners(100);
    }

    store!: T;

    init(defaultSettings: T) {
        this.store = defaultSettings;

        return this;
    }

    set<K extends keyof T, V extends T[K]>(key: K, val: V) {
        if (this.store[key] === val) return;

        const oldVal = this.store[key];

        this.store[key] = val;

        this.emit(DataStoreEvents.CHANGE, key, val, oldVal);

        return this;
    }

    get<K extends keyof T, V extends T[K]>(key: K): V {
        return this.store[key];
    }

    read(): Readonly<T> {
        return this.store;
    }

    onChange<K extends keyof T, V extends T[K]>(key: K, cb: (val: V, key: K, oldVal: V)=> void): Latiner<T, K> {
        const listener: Latiner<T, K> = (eventKey: K, val: T[K], oldVal: T[K]) => {
            if (key === eventKey) cb(val, key, oldVal);
        };

        this.on(DataStoreEvents.CHANGE, listener);

        return listener;
    }

    offChange<K extends keyof T, V extends T[K]>(key: K, listener: Latiner<T, K>) {
        this.off(DataStoreEvents.CHANGE, listener);

        return this;
    }
}
