import React, {FunctionComponent} from 'react';
import {ChatInput, ChatMessages} from 'app/components/functional';
import {controllerChats} from 'app/controllers';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {Link} from "react-router-dom";

export type PropsChat = {
    chatId: string,
}

export const Chat:FunctionComponent<PropsChat> = ({chatId}) => {
    const sendMessage = () => {
        controllerChats.sendMessage(chatId)
    }

    return <>
        <Link to={MainRoutes.LINK_CHAT(chatId)} className='chat'>
            <div>{chatId}</div>
        </Link>
        <ChatMessages />
        <ChatInput cb={sendMessage}/>
    </>


}
