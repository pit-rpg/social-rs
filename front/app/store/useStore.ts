import {DataStore} from './DataStore';
import {useState, useEffect} from 'react';

export function useCreator<T>(stor: DataStore<T>) {
    return function useStore<K extends keyof T>(key: K): [T[K], (val: T[K]) => void] {
        const valDef = stor.get(key);
        const [val, setHook] = useState(valDef);

        function set(val: T[K]) {
            stor.set(key, val);
        }

        useEffect(() => {
            const listener = stor.onChange(key, (val) => {
                setHook(val);
            });

            return function cleanup () {
                stor.offChange(key, listener);
            }
         }, []);


        return [val, set];
    }
}