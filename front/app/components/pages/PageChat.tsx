import React, {FunctionComponent, useEffect} from 'react';
import {controllerChats} from 'app/controllers';
import {Chat} from 'app/components/functional/';
import { Routes, Route, useParams } from "react-router-dom";

export const PageChat: FunctionComponent<{}> = () => {
    const { chatId } = useParams();

    useEffect(() => {
        controllerChats.getMessages(chatId!)
    }, [])

    return <>
        <Chat chatId={chatId!} />
    </>
}