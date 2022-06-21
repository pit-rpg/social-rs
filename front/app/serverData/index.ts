import {session} from './session';
import {chats} from './chats';
import {REQUEST_SDK} from './sdk';

export * from './WsSdk';

export const serverData = {
    session,
    chats,
} as const;

export const sdk = REQUEST_SDK;