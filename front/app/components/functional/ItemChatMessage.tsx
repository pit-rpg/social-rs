import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {InputText, Button} from 'app/components/basic';
import {ChatInput} from 'app/components/functional';
import {controllerChats} from 'app/controllers';
import {serverData} from 'app/serverData';
import {OutputChatMessage} from 'app/graphQL/generated_sdk';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {Link} from "react-router-dom";
import {} from 'app/controllers';

export type PropsItemChatMessage = {
    message: OutputChatMessage,
}

export const ItemChatMessage:FunctionComponent<PropsItemChatMessage> = ({message}) => {
    const [me] = useStore('me')
    let className = 'chat-message';

    if (message.edit) className += ' edit'
    if (message.user === me?.id) className += ' my'

    return <div className={className}>
        {message.message}
        {message.edit ? <div>edited</div> : null}
    </div>
}
