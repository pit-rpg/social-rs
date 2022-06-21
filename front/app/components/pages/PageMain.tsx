import {store, useStore} from 'app/store';
import React, { FunctionComponent, useState } from 'react';
import {InputText, Button} from 'app/components/basic';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {serverData} from 'app/serverData';
import {Link} from "react-router-dom";

export const PageMain:FunctionComponent<{}> = () => {
    const [user] = useStore('me');

    function logOut() {
        serverData.session.logOut();
    }

    return <>
        <Link to={MainRoutes.SETTINGS}>settings</Link>
        <br/>
        <Link to={MainRoutes.ADMIN}>admin</Link>
        <br/>
        <Link to={MainRoutes.CHATS}>chats</Link>
        <br/>
        <Link to={MainRoutes.SEARCH_USER}>user search</Link>
        <br/>
        <Button value='Logout' cb={logOut}/>
    </>;
}