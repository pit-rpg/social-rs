import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {controllerChats} from 'app/controllers';
import {InputText, Button} from 'app/components/basic';
import {serverData} from 'app/serverData';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {Link} from "react-router-dom";
import {} from 'app/controllers';


export type PropsChatInput = {
    cb: () => void;
};

export const ChatInput:FunctionComponent<PropsChatInput> = ({cb}) => {

    return <div className='chat-input'>
        <InputText filed='chats:messageInput' cb={cb}/>

        <Button value='send' cb={cb}/>
    </div>;
}
