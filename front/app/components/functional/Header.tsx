import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {InputText, Button} from 'app/components/basic';
import {serverData} from 'app/serverData';
import {BrowserRouter, Routes, Route, Link} from "react-router-dom";
import {MainRoutes} from "app/components/pages/MainRoutes";

export const Header:FunctionComponent<{}> = () => {
    const [me] = useStore('me');

    const name = me!.nameDisplay || me!.nameUser;

    return <header className='component-header'>
        <Link to={MainRoutes.INDEX}>{name}</Link>
    </header>;
}
