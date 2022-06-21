import React, {FunctionComponent} from 'react';
import {Tabs, TabsProps} from 'app/components/basic';
import {Register, Login} from 'app/components/functional';

export const PageLogin:FunctionComponent<{ initial?: number }> = ({ initial = 0 }) => {
    const tabsProps: TabsProps = {
        filed: 'login:tab',
        tabs: [
            {id: 'register', name: 'Register', element: <Register/>},
            {id: 'login', name: 'Login', element: <Login/>},
        ]
    };

    return <Tabs {...tabsProps}/>
}
