import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {InputText, Button} from 'app/components/basic';
import {serverData} from 'app/serverData';
import {OutputChat} from 'app/graphQL/generated_sdk';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {Link} from "react-router-dom";
import {} from 'app/controllers';

export type PropsItemChat = {
    chat: OutputChat,
}

export const ItemChat:FunctionComponent<PropsItemChat> = ({chat}) => {
    return <Link to={MainRoutes.LINK_CHAT(chat.id)} className='chat'>
        <div>{chat.id}</div>
    </Link>;
}
