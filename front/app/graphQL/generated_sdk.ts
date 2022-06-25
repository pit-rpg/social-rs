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
  /** A scalar that can represent any JSON Object value. */
  JSONObject: any;
};

export type Chat = {
  __typename?: 'Chat';
  chatType?: Maybe<ChatType>;
  id?: Maybe<Scalars['String']>;
  lastMsg?: Maybe<Scalars['String']>;
  name?: Maybe<Scalars['String']>;
  owner?: Maybe<Scalars['String']>;
  userLastMsg?: Maybe<Scalars['JSONObject']>;
  users?: Maybe<Array<Scalars['String']>>;
};

export type ChatConnection = {
  __typename?: 'ChatConnection';
  /** A list of edges. */
  edges: Array<ChatEdge>;
  /** A list of nodes. */
  nodes: Array<Chat>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type ChatEdge = {
  __typename?: 'ChatEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: Chat;
};

export type ChatMessage = {
  __typename?: 'ChatMessage';
  chat?: Maybe<Scalars['String']>;
  edit?: Maybe<Scalars['Int']>;
  id?: Maybe<Scalars['String']>;
  message?: Maybe<Scalars['String']>;
  user?: Maybe<Scalars['String']>;
};

export type ChatMessageChange = {
  __typename?: 'ChatMessageChange';
  change: SubscriptionChange;
  message: ChatMessage;
};

export type ChatMessageConnection = {
  __typename?: 'ChatMessageConnection';
  /** A list of edges. */
  edges: Array<ChatMessageEdge>;
  /** A list of nodes. */
  nodes: Array<ChatMessage>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type ChatMessageEdge = {
  __typename?: 'ChatMessageEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: ChatMessage;
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
  createPrivate?: Maybe<Chat>;
  createUserPrivate?: Maybe<Chat>;
  getChats: ChatConnection;
  getMessages: ChatMessageConnection;
  removeChat?: Maybe<Scalars['Boolean']>;
  removeMessages?: Maybe<Scalars['Boolean']>;
  sendMessage?: Maybe<ChatMessage>;
};


export type QueryChatCreatePrivateArgs = {
  userId: Scalars['String'];
};


export type QueryChatGetChatsArgs = {
  after?: InputMaybe<Scalars['String']>;
  before?: InputMaybe<Scalars['String']>;
  first?: InputMaybe<Scalars['Int']>;
  last?: InputMaybe<Scalars['Int']>;
};


export type QueryChatGetMessagesArgs = {
  after?: InputMaybe<Scalars['String']>;
  before?: InputMaybe<Scalars['String']>;
  chat: Scalars['String'];
  first?: InputMaybe<Scalars['Int']>;
  last?: InputMaybe<Scalars['Int']>;
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
  findUser: Array<User>;
  getUser: User;
  logIn: User;
  logOut: Scalars['Boolean'];
  register: User;
  user?: Maybe<User>;
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
  watchMessages: ChatMessageChange;
};


export type RootSubscriptionIntervalArgs = {
  n?: Scalars['Int'];
};


export type RootSubscriptionWatchMessagesArgs = {
  chat: Scalars['String'];
};

export enum SubscriptionChange {
  Delete = 'DELETE',
  New = 'NEW',
  Update = 'UPDATE'
}

export type User = {
  __typename?: 'User';
  age?: Maybe<Scalars['Int']>;
  gender?: Maybe<Gender>;
  id?: Maybe<Scalars['String']>;
  mail?: Maybe<Scalars['String']>;
  nameDisplay?: Maybe<Scalars['String']>;
  nameUser?: Maybe<Scalars['String']>;
};

export type SessionQueryVariables = Exact<{ [key: string]: never; }>;


export type SessionQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', user?: { __typename?: 'User', id?: string | null, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null } | null } };

export type GetChatsQueryVariables = Exact<{
  first?: InputMaybe<Scalars['Int']>;
  after?: InputMaybe<Scalars['String']>;
}>;


export type GetChatsQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', getChats: { __typename?: 'ChatConnection', pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null }, edges: Array<{ __typename?: 'ChatEdge', cursor: string, node: { __typename?: 'Chat', id?: string | null, owner?: string | null, users?: Array<string> | null, chatType?: ChatType | null, name?: string | null } }> } } };

export type GetMessagesQueryVariables = Exact<{
  chat: Scalars['String'];
  first?: InputMaybe<Scalars['Int']>;
  after?: InputMaybe<Scalars['String']>;
}>;


export type GetMessagesQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', getMessages: { __typename?: 'ChatMessageConnection', pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null }, edges: Array<{ __typename?: 'ChatMessageEdge', cursor: string, node: { __typename?: 'ChatMessage', id?: string | null, chat?: string | null, user?: string | null, edit?: number | null, message?: string | null } }> } } };

export type FindUserQueryVariables = Exact<{
  nameUser: Scalars['String'];
  limit?: InputMaybe<Scalars['Int']>;
}>;


export type FindUserQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', findUser: Array<{ __typename?: 'User', id?: string | null, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null, mail?: string | null, age?: number | null }> } };

export type GetUserQueryVariables = Exact<{
  id: Scalars['String'];
}>;


export type GetUserQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', getUser: { __typename?: 'User', id?: string | null, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null, mail?: string | null, age?: number | null } } };

export type LogInQueryVariables = Exact<{
  nameUser: Scalars['String'];
  password: Scalars['String'];
}>;


export type LogInQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', logIn: { __typename?: 'User', id?: string | null, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null } } };

export type RegisterQueryVariables = Exact<{
  nameUser: Scalars['String'];
  password: Scalars['String'];
}>;


export type RegisterQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', register: { __typename?: 'User', id?: string | null, nameUser?: string | null, nameDisplay?: string | null, gender?: Gender | null } } };

export type LogOutQueryVariables = Exact<{ [key: string]: never; }>;


export type LogOutQuery = { __typename?: 'RootQuery', session: { __typename?: 'QuerySession', logOut: boolean } };

export type SendMessageQueryVariables = Exact<{
  chat: Scalars['String'];
  message: Scalars['String'];
}>;


export type SendMessageQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', sendMessage?: { __typename?: 'ChatMessage', id?: string | null, chat?: string | null, user?: string | null, edit?: number | null, message?: string | null } | null } };

export type CreateChatQueryVariables = Exact<{
  userId: Scalars['String'];
}>;


export type CreateChatQuery = { __typename?: 'RootQuery', chat: { __typename?: 'QueryChat', createPrivate?: { __typename?: 'Chat', id?: string | null, owner?: string | null, users?: Array<string> | null, chatType?: ChatType | null, name?: string | null } | null } };

export type MonitorChatSubscriptionVariables = Exact<{
  chat: Scalars['String'];
}>;


export type MonitorChatSubscription = { __typename?: 'RootSubscription', watchMessages: { __typename?: 'ChatMessageChange', change: SubscriptionChange, message: { __typename?: 'ChatMessage', id?: string | null, user?: string | null, edit?: number | null, message?: string | null } } };


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
export const MonitorChatDocument = gql`
    subscription MonitorChat($chat: String!) {
  watchMessages(chat: $chat) {
    change
    message {
      id
      user
      edit
      message
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
    },
    MonitorChat(variables: MonitorChatSubscriptionVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<MonitorChatSubscription> {
      return withWrapper((wrappedRequestHeaders) => client.request<MonitorChatSubscription>(MonitorChatDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'MonitorChat', 'subscription');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;