import {store} from '../store';
import {REQUEST_SDK} from './sdk';

export const session = {
    async init() {
        const res = await REQUEST_SDK.Session();

        store.set('me', res.session.user);
    },

    async login(nameUser: string, password: string) {
        store.set('login:err', undefined);

        const promise = REQUEST_SDK.LogIn({
            nameUser,
            password
        });

        promise.catch(err => store.set('login:err', err.message));

        const res = await promise;
        const me = res.session?.logIn || null;

        store.set('me', me);

        if (me) {
            store.set('login:user', '');
            store.set('login:pass', '');
        }
    },

    async register(nameUser: string, password: string) {
        store.set('login:err', undefined);

        const promise = REQUEST_SDK.Register({
            nameUser,
            password
        });

        promise.catch(err => store.set('login:err', err.message));

        const res = await promise;
        const me = res.session?.register || null;

        store.set('me', me);

        if (me) {
            store.set('login:user', '');
            store.set('login:pass', '');
        }
    },

    async logOut() {
        store.set('login:err', undefined);

        const promise = REQUEST_SDK.LogOut();
        promise.catch(err => store.set('login:err', err.message));

        await promise;

        store.set('me', null);
    }
} as const;



