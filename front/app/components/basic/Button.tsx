import React, {FunctionComponent, DetailedHTMLProps, InputHTMLAttributes} from 'react';

export type ButtonProps = {
    cb: () => void;
    value: string
};
// export type ButtonProps = React.DetailedHTMLProps<React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>;

export const Button:FunctionComponent<ButtonProps> = ({cb, value}) => {
    return <button onClick={cb}>
        {value}
    </button>;
}