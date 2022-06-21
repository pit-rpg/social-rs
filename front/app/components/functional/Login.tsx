import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {InputText, Button} from 'app/components/basic';
import {serverData} from 'app/serverData';

export const Login:FunctionComponent<{}> = () => {
    const [user] = useStore('login:user');
    const [pass] = useStore('login:pass');
    const [err] = useStore('login:err');

    function logIn () {
        serverData.session.login(user, pass);
    }

    return <>
        <h2>Login:</h2>
        <InputText filed='login:user' focus={true} cb={logIn} />
        <br/>
        <InputText filed='login:pass' cb={logIn} />
        <br/>
        <Button value={'Login'} cb={logIn}/>
        <br/>
        {err ? err: null}
    </>
}
