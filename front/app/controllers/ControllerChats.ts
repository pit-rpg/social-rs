import {serverData, sdk} from 'app/serverData';
import {MainRoutes} from 'app/components/pages';
import {store} from 'app/store';
import {ARRAY} from '../../lib/array';
import {OutputUser, OutputChat, OutputChatMessage} from 'app/graphQL/generated_sdk';

export class ControllerChats {
    async getChats(limit = 100, after?: string) {
        const chatsResponse = await serverData.chats.getChats(limit, after);
        let chats = store.get('chats:chats');
        const nodes: OutputChat[] = chatsResponse.edges.map(e => e.node);

        chats = ARRAY.updateBy(chats, nodes, 'id');

        store.set('chats:chats', chats);
    }

    async getMessages(user: string, limit = 100, after?: string) {
        const messagesResponse = await serverData.chats.getMessages(user, limit, after);
        let messages = store.get('chats:messages');
        const nodes: OutputChatMessage[] = messagesResponse.edges.map(e => e.node);

        messages = ARRAY.updateBy(messages, nodes, 'id');

        store.set('chats:messages', messages);
    }

    async sendMessage(userId: string) {
        const text = store.get('chats:messageInput');
        store.set('chats:messageInput', "");

        const message = await serverData.chats.sendMessage(userId, text);
        const messages = store.get('chats:messages');

        messages.push(message);
        store.set('chats:messages', messages);
    }

    async createChat(userId: string) {
        const res = await sdk.CreateChat({userId});
        const chat = res.chat?.createPrivate!;
        const route = MainRoutes.LINK_CHAT(chat.id!);

        location.assign(route);
    }
}

export const controllerChats = new ControllerChats();