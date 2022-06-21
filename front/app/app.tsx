import React, {FunctionComponent, useEffect} from "react";
import {serverData} from "./serverData";
import {useStore} from "./store";
import {BrowserRouter, Routes, Route, Link} from "react-router-dom";

import {
    PageLogin,
    PageMain,
    PageSettings,
    PageAdmin,
    PageChats,
    PageChat,
    MainRoutes,
    PageSearchUser,
    PageUser,
} from "./components/pages";
import {Header} from "./components/functional/Header";

export const App: FunctionComponent<{}> = (props) => {
    const [me] = useStore('me');

    useEffect(() => {
        serverData.session.init();
    }, []);

    if (me === null) {
        return <div>
            <PageLogin></PageLogin>
        </div>;
    }

    if (me === undefined) {
        return <h3>Loading...</h3>;
    }

    return <BrowserRouter>
        <Header/>
        <Routes>
            <Route path={MainRoutes.INDEX} element={<PageMain/>}/>
            <Route path={MainRoutes.SETTINGS} element={<PageSettings/>}/>
            <Route path={MainRoutes.ADMIN} element={<PageAdmin/>}/>
            <Route path={MainRoutes.CHATS} element={<PageChats/>}/>
            <Route path={MainRoutes.ROUTE_CHAT} element={<PageChat/>}/>
            <Route path={MainRoutes.SEARCH_USER} element={<PageSearchUser/>}/>
            <Route path={MainRoutes.ROUTE_USER} element={<PageUser/>}/>
        </Routes>
    </BrowserRouter>;
}
