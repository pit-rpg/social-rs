import {User, Chat, ChatMessage} from 'app/graphQL/generated_sdk';

export type DefaultData = {
    me: User | null | undefined;

    'login:user': string;
    'login:pass': string;
    'login:err'?: string;
    'login:tab': 'login' | 'register';

    // 'chats:chatsResponse': Chat[];
    'chats:chats': Chat[];
    'chats:messageInput': string;
    'chats:messages': ChatMessage[];

    'contacts:searchInput': string;
    'contacts:searchResult': User[];
    'contacts:user'?: User,
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