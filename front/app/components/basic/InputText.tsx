import {useStore, DefaultData} from '../../store';
import React, {FunctionComponent} from 'react';

export type InputTextProps = {
    filed: keyof DefaultData,
    placeholder?: string;
    focus?: boolean;
    password?: boolean;
    cb?: () => void;
}

export const InputText:FunctionComponent<InputTextProps> = (props) => {
    const [val, setVal] = useStore(props.filed);

    const type = props.password ? 'password' : 'text';

    return <input
        value={val as string}
        onChange={e => setVal(e.target.value)}
        type={type}
        placeholder={props.placeholder}
        autoFocus={props.focus}
        onSubmit={props.cb}
    />;
}