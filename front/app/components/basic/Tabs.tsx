import React, {FunctionComponent, ReactElement} from 'react';
import {useStore, DefaultData} from '../../store';

export type TabsProps = {
    tabs: {
        id: string,
        name: string,
        element: ReactElement
    }[];
    filed: keyof DefaultData,
};

export const Tabs:FunctionComponent<TabsProps> = (props) => {
    const [currentId, setId] = useStore(props.filed);

    const tabs = props.tabs.map((item, i) => {
        return <li key={i} onClick={() => setId(item.id)} className={item.id === currentId ? 'active' : ''}>
            {item.name}
        </li>;
    });

    const currentTab = props.tabs.find(e => e.id === currentId);

    if (!currentTab) {
        throw new Error(`cent find tab: ${currentId}`);
    }

    return <div>
        <ul>{tabs}</ul>
        {currentTab.element}
    </div>;
}