export const MainRoutes = {
    INDEX: '/',
    ADMIN: '/admin',
    SETTINGS: '/settings',
    CHATS: '/chats',
    SEARCH_USER: '/search_user',

    ROUTE_CHAT: '/chat/:chatId',
    LINK_CHAT(chatId: string) {return '/chat/' + chatId},

    ROUTE_USER: '/user/:userId',
    LINK_USER(userId: string) {return '/user/' + userId},


} as const;