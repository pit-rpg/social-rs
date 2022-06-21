import {sdk} from 'app/serverData';
import {store} from 'app/store';
import debounce from 'debounce';

export class ControllerContacts {
    readonly debounceFindUser: () => void;

    constructor() {
        this.debounceFindUser = debounce(this.findUser.bind(this), 500);
    }

    async findUser() {
        let search = store.get('contacts:searchInput');

        if (!search?.length ) return;

        const res = await sdk.FindUser({nameUser: search});
        const users = res.session?.findUser || [];
        store.set('contacts:searchResult', users);
    }

    async getUser(id: string) {
        store.set('contacts:user', undefined);

        const res = await sdk.GetUser({id});
        const user = res.session.getUser || undefined;

        store.set('contacts:user', user);
    }
}

export const controllerContacts = new ControllerContacts();