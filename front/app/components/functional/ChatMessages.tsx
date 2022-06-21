import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {ItemChatMessage} from 'app/components/functional';

export const ChatMessages:FunctionComponent<{}> = () => {
    const [messages] = useStore('chats:messages');

    return <div className='messages'>
        {messages.map(msg => <ItemChatMessage message={msg} key={msg.id} />)}
    </div>
}
