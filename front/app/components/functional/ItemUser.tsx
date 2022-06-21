import React, {FunctionComponent} from 'react';
import {OutputUser} from 'app/graphQL/generated_sdk';
import {MainRoutes} from 'app/components/pages/MainRoutes';
import {Link} from "react-router-dom";
import {} from 'app/controllers';

export type PropsItemUser = {
    user: OutputUser,
}

export const ItemUser:FunctionComponent<PropsItemUser> = ({user}) => {
    const nameDisplay = user.nameDisplay ? <div>{user.nameDisplay}</div> : null;

    return <Link to={MainRoutes.LINK_USER(user.id)} className='chat'>
        <div>{user.nameUser}</div>
        {nameDisplay}
    </Link>;
}
