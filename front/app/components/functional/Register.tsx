import {useStore} from 'app/store';
import React, {FunctionComponent} from 'react';
import {InputText, Button} from 'app/components/basic';
import {serverData} from 'app/serverData';

export const Register: FunctionComponent<{}> = () => {
    const [user] = useStore('login:user');
    const [pass] = useStore('login:pass');
    const [err] = useStore('login:err');

    function register () {
        serverData.session.register(user, pass);
    }

    return <>
        <h2>Registration:</h2>
        <InputText filed='login:user' focus={true} cb={register} />
        <br/>
        <InputText filed='login:pass' cb={register}/>
        <br/>
        <Button value={'Register'} cb={register}/>
        <br/>
        {err ? err: null}
    </>
}
