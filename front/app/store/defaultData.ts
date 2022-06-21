import {OutputUser, OutputChat, OutputChatMessage} from 'app/graphQL/generated_sdk';

export type DefaultData = {
    me: OutputUser | null | undefined;

    'login:user': string;
    'login:pass': string;
    'login:err'?: string;
    'login:tab': 'login' | 'register';

    // 'chats:chatsResponse': Chat[];
    'chats:chats': OutputChat[];
    'chats:messageInput': string;
    'chats:messages': OutputChatMessage[];

    'contacts:searchInput': string;
    'contacts:searchResult': OutputUser[];
    'contacts:user'?: OutputUser,
};

export type DefaultDataKeys = keyof DefaultData;

export const defaultData: DefaultData = {
    me: undefined,

    'login:user': '',
    'login:pass': '',
    'login:tab': 'register',

    // 'chats:chatsResponse': [],
    'chats:chats': [],
    'chats:messageInput': '',
    'chats:messages': [],

    'contacts:searchInput': '',
    'contacts:searchResult': [],
};