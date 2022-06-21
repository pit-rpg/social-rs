import React, {FunctionComponent, useEffect} from 'react';
import { Routes, Route, useParams } from "react-router-dom";
import {controllerContacts, controllerChats} from "app/controllers";
import {Button} from "app/components/basic";
import {useStore} from "app/store";

export const PageUser:FunctionComponent<{}> = () => {
    const { userId } = useParams();
    const [user] = useStore('contacts:user');

    useEffect(() => {
        controllerContacts.getUser(userId!)
    }, [userId])

    if (!user) {
        return <div>loading...</div>
    }

    function createChat() {
        controllerChats.createChat(userId!)
    }

    return <>
        <h2>{user.nameUser}</h2>
        <h3>{user.nameDisplay}</h3>
        <h5>{user.id}</h5>
        <h3>{user.gender}</h3>
        <Button cb={createChat} value='create chat'/>
    </>;
}