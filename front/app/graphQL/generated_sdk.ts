import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export enum ChatType {
  Group = 'GROUP',
  Private = 'PRIVATE',
  UserPrivate = 'USER_PRIVATE'
}

export enum Gender {
  Female = 'FEMALE',
  Male = 'MALE',
  None = 'NONE',
  Other = 'OTHER'
}

export type InputFindUser = {
  limit?: InputMaybe<Scalars['Int']>;
  nameUser: Scalars['String'];
};

export type InputUserLogin = {
  nameUser: Scalars['String'];
  password: Scalars['String'];
};

export type OutputChat = {
  __typename?: 'OutputChat';
  chatType?: Maybe<ChatType>;
  id: Scalars['String'];
  name?: Maybe<Scalars['String']>;
  owner?: Maybe<Scalars['String']>;
  users?: Maybe<Array<Scalars['String']>>;
};

export type OutputChatConnection = {
  __typename?: 'OutputChatConnection';
  /** A list of edges. */
  edges: Array<OutputChatEdge>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type OutputChatEdge = {
  __typename?: 'OutputChatEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: OutputChat;
};

export type OutputChatMessage = {
  __typename?: 'OutputChatMessage';
  chat?: Maybe<Scalars['String']>;
  edit?: Maybe<Scalars['Int']>;
  id: Scalars['String'];
  message?: Maybe<Scalars['String']>;
  user?: Maybe<Scalars['String']>;
};

export type OutputChatMessageConnection = {
  __typename?: 'OutputChatMessageConnection';
  /** A list of edges. */
  edges: Array<OutputChatMessageEdge>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type OutputChatMessageEdge = {
  __typename?: 'OutputChatMessageEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: OutputChatMessage;
};

export type OutputUser = {
  __typename?: 'OutputUser';
  age?: Maybe<Scalars['Int']>;
  gender?: Maybe<Gender>;
  id: Scalars['String'];
  mail?: Maybe<Scalars['String']>;
  nameDisplay?: Maybe<Scalars['String']>;
  nameUser?: Maybe<Scalars['String']>;
};

/** Information about pagination in a connection */
export type PageInfo = {
  __typename?: 'PageInfo';
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']>;
};

export type QueryChat = {
  __typename?: 'QueryChat';
  createPrivate?: Maybe<OutputChat>;
  createUserPrivate?: Maybe<OutputChat>;
  getChats: OutputChatConnection;
  getMessages: OutputChatMessageConnection;
  removeChat?: Maybe<Scalars['Boolean']>;
  removeMessages?: Maybe<Scalars['Boolean']>;
  sendMessage?: Maybe<OutputChatMessage>;
};


export type QueryChatCreatePrivateArgs = {
  userId: Scalars['String'];
};


export type QueryChatGetChatsArgs = {
  after?: InputMaybe<Scalars['String']>;
  before?: InputMaybe<Scalars['String']>;
  first?: InputMaybe<Scalars['Int']>;
};


export type QueryChatGetMessagesArgs = {
  after?: InputMaybe<Scalars['String']>;
  before?: InputMaybe<Scalars['String']>;
  chat: Scalars['String'];
  first?: InputMaybe<Scalars['Int']>;
};


export type QueryChatRemoveChatArgs = {
  chat: Scalars['String'];
};


export type QueryChatRemoveMessagesArgs = {
  chat: Scalars['String'];
  messages: Array<Scalars['String']>;
};


export type QueryChatSendMessageArgs = {
  chat: Scalars['String'];
  message: Scalars['String'];
};

export type QuerySession = {
  __typename?: 'QuerySession';
  findUser: Array<OutputUser>;
  getUser: OutputUser;
  logIn: OutputUser;
  logOut: Scalars['Boolean'];
  register: OutputUser;
  user?: Maybe<OutputUser>;
};


export type QuerySessionFindUserArgs = {
  data: InputFindUser;
};


export type QuerySessionGetUserArgs = {
  id: Scalars['String'];
};


export type QuerySessionLogInArgs = {
  data: InputUserLogin;
};


export type QuerySessionRegisterArgs = {
  data: InputUserLogin;
};

export type RootQuery = {
  __typename?: 'RootQuery';
  chat: QueryChat;
  session: QuerySession;
};

export type RootSubscription = {
  __typename?: 'RootSubscription';
  interval: Scalars['Int'];
  values: Scalars['Int'];
};


export type RootSubscriptionIntervalArgs = {
  n?: Scalars['Int'];
};

export type SessionQueryVariables = Exact<{ [key: string]: never; }>;


export type SessionQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', user?: { __typename?: 'OutputUser', id: string, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null } | null } };

export type GetChatsQueryVariables = Exact<{
  first?: InputMaybe<Scalars['Int']>;
  after?: InputMaybe<Scalars['String']>;
}>;


export type GetChatsQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', getChats: { __typename?: 'OutputChatConnection', pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null }, edges: Array<{ __typename?: 'OutputChatEdge', cursor: string, node: { __typename?: 'OutputChat', id: string, owner?: string | null, users?: Array<string> | null, chatType?: ChatType | null, name?: string | null } }> } } };

export type GetMessagesQueryVariables = Exact<{
  chat: Scalars['String'];
  first?: InputMaybe<Scalars['Int']>;
  after?: InputMaybe<Scalars['String']>;
}>;


export type GetMessagesQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', getMessages: { __typename?: 'OutputChatMessageConnection', pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null }, edges: Array<{ __typename?: 'OutputChatMessageEdge', cursor: string, node: { __typename?: 'OutputChatMessage', id: string, chat?: string | null, user?: string | null, edit?: number | null, message?: string | null } }> } } };

export type FindUserQueryVariables = Exact<{
  nameUser: Scalars['String'];
  limit?: InputMaybe<Scalars['Int']>;
}>;


export type FindUserQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', findUser: Array<{ __typename?: 'OutputUser', id: string, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null, mail?: string | null, age?: number | null }> } };

export type GetUserQueryVariables = Exact<{
  id: Scalars['String'];
}>;


export type GetUserQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', getUser: { __typename?: 'OutputUser', id: string, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null, mail?: string | null, age?: number | null } } };

export type LogInQueryVariables = Exact<{
  nameUser: Scalars['String'];
  password: Scalars['String'];
}>;


export type LogInQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', logIn: { __typename?: 'OutputUser', id: string, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null } } };

export type RegisterQueryVariables = Exact<{
  nameUser: Scalars['String'];
  password: Scalars['String'];
}>;


export type RegisterQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', register: { __typename?: 'OutputUser', id: string, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null } } };

export type LogOutQueryVariables = Exact<{ [key: string]: never; }>;


export type LogOutQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', logOut: boolean } };

export type SendMessageQueryVariables = Exact<{
  chat: Scalars['String'];
  message: Scalars['String'];
}>;


export type SendMessageQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', sendMessage?: { __typename?: 'OutputChatMessage', id: string, chat?: string | null, user?: string | null, edit?: number | null, message?: string | null } | null } };

export type CreateChatQueryVariables = Exact<{
  userId: Scalars['String'];
}>;


export type CreateChatQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', createPrivate?: { __typename?: 'OutputChat', id: string, owner?: string | null, users?: Array<string> | null, chatType?: ChatType | null, name?: string | null } | null } };


export const SessionDocument = gql`
    query Session {
  session {
    user {
      id
      nameUser
      nameDisplay
      gender
    }
  }
}
    `;
export const GetChatsDocument = gql`
    query GetChats($first: Int, $after: String) {
  chat {
    getChats(first: $first, after: $after) {
      pageInfo {
        hasPreviousPage
        hasNextPage
        startCursor
        endCursor
      }
      edges {
        cursor
        node {
          id
          owner
          users
          chatType
          name
        }
      }
    }
  }
}
    `;
export const GetMessagesDocument = gql`
    query GetMessages($chat: String!, $first: Int, $after: String) {
  chat {
    getMessages(chat: $chat, first: $first, after: $after) {
      pageInfo {
        hasPreviousPage
        hasNextPage
        startCursor
        endCursor
      }
      edges {
        cursor
        node {
          id
          chat
          user
          edit
          message
        }
      }
    }
  }
}
    `;
export const FindUserDocument = gql`
    query FindUser($nameUser: String!, $limit: Int) {
  session {
    findUser(data: {nameUser: $nameUser, limit: $limit}) {
      id
      nameUser
      nameDisplay
      gender
      mail
      age
    }
  }
}
    `;
export const GetUserDocument = gql`
    query GetUser($id: String!) {
  session {
    getUser(id: $id) {
      id
      nameUser
      nameDisplay
      gender
      mail
      age
    }
  }
}
    `;
export const LogInDocument = gql`
    query LogIn($nameUser: String!, $password: String!) {
  session {
    logIn(data: {nameUser: $nameUser, password: $password}) {
      id
      nameUser
      nameDisplay
      gender
    }
  }
}
    `;
export const RegisterDocument = gql`
    query Register($nameUser: String!, $password: String!) {
  session {
    register(data: {nameUser: $nameUser, password: $password}) {
      id
      nameUser
      nameDisplay
      gender
    }
  }
}
    `;
export const LogOutDocument = gql`
    query LogOut {
  session {
    logOut
  }
}
    `;
export const SendMessageDocument = gql`
    query SendMessage($chat: String!, $message: String!) {
  chat {
    sendMessage(chat: $chat, message: $message) {
      id
      chat
      user
      edit
      message
    }
  }
}
    `;
export const CreateChatDocument = gql`
    query CreateChat($userId: String!) {
  chat {
    createPrivate(userId: $userId) {
      id
      owner
      users
      chatType
      name
    }
  }
}
    `;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    Session(variables?: SessionQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<SessionQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<SessionQuery>(SessionDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'Session', 'query');
    },
    GetChats(variables?: GetChatsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<GetChatsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<GetChatsQuery>(GetChatsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'GetChats', 'query');
    },
    GetMessages(variables: GetMessagesQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<GetMessagesQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<GetMessagesQuery>(GetMessagesDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'GetMessages', 'query');
    },
    FindUser(variables: FindUserQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<FindUserQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<FindUserQuery>(FindUserDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'FindUser', 'query');
    },
    GetUser(variables: GetUserQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<GetUserQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<GetUserQuery>(GetUserDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'GetUser', 'query');
    },
    LogIn(variables: LogInQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<LogInQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<LogInQuery>(LogInDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'LogIn', 'query');
    },
    Register(variables: RegisterQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<RegisterQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<RegisterQuery>(RegisterDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'Register', 'query');
    },
    LogOut(variables?: LogOutQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<LogOutQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<LogOutQuery>(LogOutDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'LogOut', 'query');
    },
    SendMessage(variables: SendMessageQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<SendMessageQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<SendMessageQuery>(SendMessageDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'SendMessage', 'query');
    },
    CreateChat(variables: CreateChatQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<CreateChatQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<CreateChatQuery>(CreateChatDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'CreateChat', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;