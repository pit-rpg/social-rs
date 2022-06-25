import React, {FunctionComponent, useEffect} from 'react';
import {controllerChats} from 'app/controllers';
import {useStore, store} from 'app/store';
import {WsSdk} from 'app/serverData';
import {Chat} from 'app/components/functional/';
import { Routes, Route, useParams } from "react-router-dom";

export const PageChat: FunctionComponent<{}> = () => {
    const { chatId } = useParams();

    useEffect(() => {
        controllerChats.getMessages(chatId!)

        const closeConnection = controllerChats.subscribeToMessages(chatId!);
        return () => {
            closeConnection();
            store.set('chats:messages', []);
        }
    }, [])

    return <>
        <Chat chatId={chatId!} />
    </>
}