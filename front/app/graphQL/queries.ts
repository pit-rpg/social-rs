import { gql } from "graphql-request";

/**
 * Queries
 */

export const querySession = gql`
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

export const queryGetChats = gql`
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

export const queryGetMessages = gql`
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

export const queryFindUser = gql`
  query FindUser($nameUser: String!, $limit: Int) {
    session {
      findUser(data: { nameUser: $nameUser, limit: $limit }) {
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

export const queryGetUser = gql`
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

/**
 * Mutations
 */

export const mutationLogIn = gql`
  query LogIn($nameUser: String!, $password: String!) {
    session {
      logIn(data: { nameUser: $nameUser, password: $password }) {
        id
        nameUser
        nameDisplay
        gender
      }
    }
  }
`;

export const mutationRegister = gql`
  query Register($nameUser: String!, $password: String!) {
    session {
      register(data: { nameUser: $nameUser, password: $password }) {
        id
        nameUser
        nameDisplay
        gender
      }
    }
  }
`;

export const mutationLogout = gql`
  query LogOut {
    session {
      logOut
    }
  }
`;

export const mutationSendMessage = gql`
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

export const mutationCreateChat = gql`
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


export const subscriptionMonitorChat = gql`subscription MonitorChat($chat: String!) {
    watchMessages(chat: $chat) {
      change, message {
        id, user, edit, message
      }
    }
}`;



// export const SubscriptionCountToNumber = gql`
//   subscription CountToNumber($count: Int) {
//     countToNumber(count: $count)
//   }
// `;
