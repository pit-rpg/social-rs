import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {ChatMessage} from 'app/graphQL/generated_sdk';

export type PropsItemChatMessage = {
    message: ChatMessage,
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
