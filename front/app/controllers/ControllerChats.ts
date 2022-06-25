import {serverData, sdk, WsSdk} from 'app/serverData';
import {MainRoutes} from 'app/components/pages';
import {store} from 'app/store';
import {ARRAY} from '../../lib/array';
import {Chat, ChatMessage, SubscriptionChange} from 'app/graphQL/generated_sdk';

export class ControllerChats {
    async getChats(limit = 100, after?: string) {
        const chatsResponse = await serverData.chats.getChats(limit, after);
        let chats = store.get('chats:chats') as Required<Chat>[];
        const nodes = chatsResponse.edges.map(e => e.node) as Required<Chat>[];

        chats = ARRAY.updateBy(chats, nodes, 'id');

        store.set('chats:chats', chats);
    }

    async getMessages(user: string, limit = 100, after?: string) {
        const messagesResponse = await serverData.chats.getMessages(user, limit, after);
        let messages = store.get('chats:messages') as Required<ChatMessage>[];
        const nodes = messagesResponse.edges.map(e => e.node) as Required<ChatMessage>[];

        messages = ARRAY.updateBy(messages, nodes, 'id');

        store.set('chats:messages', messages);
    }

    subscribeToMessages(chatId: string) {
        controllerChats.getMessages(chatId!)
        const connection = new WsSdk('MonitorChat', {chat: chatId});

        connection.on('next', data => {
            console.log('NEXT!!!', data);

            const {change, message} = data.watchMessages;
            let messages = store.get('chats:messages').slice() as Required<ChatMessage>[];

            if (change === SubscriptionChange.New || change === SubscriptionChange.Update) {
                this.addUpdateMessage(message, messages);
            }
            if (change === SubscriptionChange.Delete) {
                ARRAY.removeWith(messages, e => e.id === message.id);
            }

            // ARRAY.sortBy(messages, 'id');
            // messages.sort((a, b) => a.id! > b.id! ? 1: -1);
            store.set('chats:messages', messages);
        });

        return () => connection.close();
    }

    async sendMessage(userId: string) {
        const text = store.get('chats:messageInput');
        store.set('chats:messageInput', "");

        const message = await serverData.chats.sendMessage(userId, text);
        const messages = store.get('chats:messages').slice();

        this.addUpdateMessage(message, messages);

        store.set('chats:messages', messages);
    }

    async createChat(userId: string) {
        const res = await sdk.CreateChat({userId});
        const chat = res.chat?.createPrivate!;
        const route = MainRoutes.LINK_CHAT(chat.id!);

        location.assign(route);
    }

    private addUpdateMessage(msg: ChatMessage, messages: ChatMessage[]) {
        const old = messages.find(m => m.id === msg.id);
        msg = {...old, ...msg};
        ARRAY.removeWith(messages, i => i.id === msg.id);
        messages.push(msg);
        ARRAY.sortBy(messages as Required<ChatMessage>[], 'id');
    }
}

export const controllerChats = new ControllerChats();