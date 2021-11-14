import Vue from 'vue';
import {JoinInput, InputType, SelfJoinedOutput, PostInput, Message, PostedOutput} from './../types/proto';
import Router from '../router'; 
class SocketService extends Vue {
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

    send_message(content: string) {
        const newMessage = {
            input: InputType.Post,
            payload: {
                content
            }
        };
        const stringInput = JSON.stringify(newMessage);

        const postedMessage: Message = {
            user: this.client_id,
            content: content
        };
        Vue.prototype.$feed.push(postedMessage);

        Vue.prototype.$ws.send(stringInput);
    }

    handle_message(msg: MessageEvent) {

        if (msg.data.includes("SelfJoined")) {
            const joinedOutput: SelfJoinedOutput = JSON.parse(msg.data);
            this.nickname = joinedOutput.output.payload.nickname;
            this.client_id = joinedOutput.output.payload.client_id;
            Router.push('feed');
        }

        if (msg.data.includes("\"output\":\"Posted\"")) {
            console.log(msg.data)
            const postedMessage: PostedOutput = JSON.parse(msg.data);
            const newMessage: Message = {
                user: postedMessage.client_id,
                content: postedMessage.output.payload.content
            };
            Vue.prototype.$feed.push(newMessage);
        }
    }
}

export default SocketService;
