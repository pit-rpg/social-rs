import React, {FunctionComponent, useEffect} from 'react';
import {controllerChats} from 'app/controllers';
import {Chats} from 'app/components/functional/Chats';
import { Routes, Route, useParams } from "react-router-dom";

export const PageChats: FunctionComponent<{}> = () => {
    useEffect(() => {
        controllerChats.getChats();
    }, []);

    return <>
        <h1>CHATS</h1>
        <Chats/>
    </>
}