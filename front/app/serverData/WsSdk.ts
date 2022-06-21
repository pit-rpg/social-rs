import { getSdk } from 'app/graphQL/generated_sdk';
import { DocumentNode } from 'graphql';
import { createClient } from 'graphql-ws';


type QueueItem<T> = {
    last: boolean
    res: (res: T) => void;
    rej: (err: Error) => void;
    promise: Promise<T>;
}

export class WsSdk <
    R extends ReturnType<typeof getSdk>,
    K extends keyof R,
    F extends R[K],
    P = F extends (arg: infer P) => any ? P : never,
    X = F extends (arg: P) => any ? ReturnType<F> : never,
    XR = Awaited<X>,
>
{
    static clientStub = {
        request(document: any, variables?: any, requestHeaders?: any) {
            return {document, variables, requestHeaders};
        }
    };

    static client = createClient({
        url: `ws://${location.host}/graphqlWs`,
    });

    private _cleanup!: () => void
    public readonly queue: QueueItem<XR>[] = [];

    constructor(name: K, variables: P) {
        const sdkWs = (getSdk as any)(WsSdk.clientStub as any, (action: any, operationName: any, operationType: any) => {
            const {document, variables, requestHeaders} = action() as any;
            return {operationName, operationType, document};
        }) as any;

        const {operationName, operationType, document} = sdkWs[name](variables);
        const query = (document as DocumentNode).loc!.source.body;

        let nextItem = this.getNextResolver()


        this._cleanup = WsSdk.client.subscribe(
            {
                query,
                variables: variables as any,
                operationName: operationName,

            },
            {
                next: ({data}) => {
                    nextItem.res(data as XR);
                    nextItem = this.getNextResolver()
                },
                error: (err: Error) => {
                    nextItem.rej(err);
                },
                complete() {
                    nextItem.last = true;
                    nextItem.res(undefined!);
                },
            },
        );
    }

    close() {
        this._cleanup();
    }

    async *[Symbol.asyncIterator](): AsyncGenerator<XR, any, any> {
        while (true) {
            const item = this.queue.shift()!;
            const value = (await item?.promise)!;

            if (item.last) break

            yield value;
        }

        return;
    }

    private getNextResolver() {
        const item: QueueItem<XR> = {
            last: false,
            res: undefined as any as (res: XR) => void,
            rej: undefined as any as (err: Error) => void,
            promise: undefined as any
        };

        item.promise = new Promise<XR>((res, rej) => {
            item.res = res;
            item.rej = rej;
        })

        this.queue.push(item);

        return item;
    }
}


// (async function test() {
//     for await(const item of new WsSdk('CountToNumber', {})) {
//         console.log(item);
//     }
//     console.log('finnnn');
// })()