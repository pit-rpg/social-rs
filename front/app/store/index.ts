export * from './DataStore';
export * from './defaultData';

import {DataStore} from './DataStore';
import {DefaultData, defaultData} from './defaultData';
import {useCreator} from './useStore';


export const store = new DataStore<DefaultData>()
    .init(defaultData);

export const useStore = useCreator(store);
