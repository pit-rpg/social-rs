import React, {FunctionComponent, useEffect} from 'react';
import {InputText} from 'app/components/basic';
import {ItemUser} from 'app/components/functional';
import {controllerContacts} from 'app/controllers';
import {useStore} from 'app/store';

export const PageSearchUser:FunctionComponent<{}> = () => {
    const [input] = useStore('contacts:searchInput');
    const [users] = useStore('contacts:searchResult');

    useEffect(() => {
        controllerContacts.debounceFindUser();
    }, [input]);

    const usersList = users.map(user => <ItemUser user={user} key={user.id}/>)

    return <>
        <h2>User search</h2>
        <InputText filed='contacts:searchInput'/>
        {usersList}
    </>;
}