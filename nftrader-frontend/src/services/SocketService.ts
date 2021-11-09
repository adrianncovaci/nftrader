import Vue from 'vue';
import {JoinInput, InputType, SelfJoinedOutput, OutputParcel} from './../types/proto';
class SocketService {
    nickname!: string;
    client_id!: string;



    login(nickname: string) {
        const input: JoinInput  = {
            input: InputType.Join,
            payload: { nickname: nickname }  
        };
        const stringInput = JSON.stringify(input);
        Vue.prototype.$ws.send(stringInput);
    }

    handle_message(msg: MessageEvent) {
        if (msg.data.includes("SelfJoined")) {
            const joinedOutput: SelfJoinedOutput = JSON.parse(msg.data);
            console.log(joinedOutput);
            this.nickname = joinedOutput.payload.nickname;
            this.client_id = joinedOutput.payload.client_id;
        }
    }
}

export default SocketService;
