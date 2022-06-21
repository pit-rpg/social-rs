import {GraphQLClient} from 'graphql-request'
import {getSdk} from 'app/graphQL/generated_sdk';

// export const client = new GraphQLClient('http://localhost:3000/graphql')
export const client = new GraphQLClient('/graphql')


// export const client = new GraphQLClient('/graphql', {headers: {'content-type': 'application/json'}})
export const REQUEST_SDK = getSdk(client)
