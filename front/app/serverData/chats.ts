import {store} from '../store';
import {REQUEST_SDK} from './sdk';


export const chats = {
    async getChats(first = 100, after?: string) {
        const res = await REQUEST_SDK.GetChats({
            first,
            after,
        });

        return res.chat.getChats || [];
    },

    async getMessages(chat: string, first = 100, after?: string) {
        const res = await REQUEST_SDK.GetMessages({
            chat,
            first,
            after,
        });

        return res.chat.getMessages || [];
    },

    async sendMessage(chat: string, message: string) {
        const res = await REQUEST_SDK.SendMessage({
            chat,
            message,
        });

        return res.chat.sendMessage!;
    },
} as const;

