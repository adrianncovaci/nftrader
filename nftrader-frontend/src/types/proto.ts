export enum InputType {
    Join = 'Joined',
    Post = 'Posted',
}

export type JoinInput = {
    input: InputType.Join;
    payload: { nickname: string; };
};

export type PostInput = {
    input: InputType.Post;
    payload: { body: string; };
};

export type Input = JoinInput | PostInput;

export enum OutputType {
    Error = 'Err',
    Joined = 'Joined',
    SelfJoined = 'SelfJoined',
    Posted = 'Posted',
    UserLeft = 'Left',
}

export enum OutputError {
    NameTaken = 'name-taken',
    InvalidName = 'invalid-name',
    NotJoined = 'not-joined',
    InvalidMessageBody = 'invalid-message-body',
}

export type OutputParcel = {
    client_id: string;
    output: Output;
}

export type ErrorOutput = {
    output: OutputType.Error;
    payload: { code: OutputError };
};

export type JoinedOutput = {
    output: OutputType.Joined;
    payload: {
        user: string;
    };
};

export type SelfJoinedOutput = {
    output: OutputType.SelfJoined;
    payload: {
        client_id: string;
        nickname: string;
    };
};

export type UserLeftOutput = {
    output: OutputType.UserLeft;
    payload: {
        userId: string;
    };
};

export type PostedOutput = {
    output: OutputType.Posted;
    payload: {
        message: string;
    };
};
export type Output =
    OutputError |
    JoinedOutput |
    UserLeftOutput |
    PostedOutput
