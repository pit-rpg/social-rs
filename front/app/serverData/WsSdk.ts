import { getSdk } from 'app/graphQL/generated_sdk';
import { DocumentNode } from 'graphql';
import { createClient } from 'graphql-ws';
import { EventEmitter } from 'events';

export class WsSdk <
    R extends ReturnType<typeof getSdk>,
    K extends keyof R,
    F extends R[K],
    P = F extends (arg: infer P) => any ? P : never,
    X = F extends (arg: P) => any ? ReturnType<F> : never,
    XR = Awaited<X>,
> extends EventEmitter
{
    private static clientStub = {
        request(document: any, variables?: any, requestHeaders?: any) {
            return {document, variables, requestHeaders};
        }
    };

    private static client = createClient({
        url: `ws://${location.host}/graphql`,
    });

    private _cleanup!: () => void

    constructor(name: K, variables: P) {
        super();

        const sdkWs = (getSdk as any)(WsSdk.clientStub as any, (action: any, operationName: any, operationType: any) => {
            const {document, variables, requestHeaders} = action() as any;
            return {operationName, operationType, document};
        }) as any;

        const {operationName, operationType, document} = sdkWs[name](variables);
        const query = (document as DocumentNode).loc!.source.body;

        this._cleanup = WsSdk.client.subscribe(
            {
                query,
                variables: variables as any,
                operationName: operationName,

            },
            {
                next: ({data}) => {
                    this.emit('next', data as XR)
                },
                error: (err: Error) => {
                    this.emit('error', err)
                },
                complete: () => {
                    this.emit('complete')
                },
            },
        );
    }

    close() {
        this._cleanup();
    }

    on(e: 'complete', f: () => void): this;
    on(e: 'error', f: (e: Error) => void): this;
    on(e: 'next', f: (d: XR) => void): this;
    on(e: string, f: (d: any) => void): this {
        return super.on(e, f);
    }
}


// (async function test() {
//     for await(const item of new WsSdk('CountToNumber', {})) {
//         console.log(item);
//     }
//     console.log('finnnn');
// })()