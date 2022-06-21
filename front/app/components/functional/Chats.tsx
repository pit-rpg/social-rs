import {useStore} from 'app/store';
import React, {FunctionComponent, useEffect} from 'react';
import {InputText, Button} from 'app/components/basic';
import {controllerChats} from 'app/controllers';
import {ItemChat, ChatInput} from 'app/components/functional';
import {serverData} from 'app/serverData';
import {} from 'app/controllers';

export const Chats:FunctionComponent<{}> = () => {
    const [chats] = useStore('chats:chats');

    return <div className='chats-list'>
        {chats.map(chat => {
            return <ItemChat chat={chat} key={chat.id}></ItemChat>
        })}
    </div>
}
