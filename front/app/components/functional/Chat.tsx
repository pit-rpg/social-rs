import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {InputText, Button} from 'app/components/basic';
import {ChatInput, ItemChatMessage, ChatMessages} from 'app/components/functional';
import {controllerChats} from 'app/controllers';
import {serverData} from 'app/serverData';
import {OutputChat} from 'app/graphQL/generated_sdk';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {Link} from "react-router-dom";
import {} from 'app/controllers';

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
