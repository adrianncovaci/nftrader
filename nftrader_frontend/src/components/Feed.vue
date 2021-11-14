<template>
    <div >
        <div class="feed" style="position:fixed; width:100%; height: 90%; top:0px;">
            <div v-for="(message, index) in this.feed()" :key="index"> 
                [{{ message.user }}] ====> {{ message.content }}
            </div>
        </div>
        <div style="position:fixed; width:100%; height: 10%; bottom:0px;">
            <v-card
               class="mx-auto"
             >
               <v-card-text>
                   <v-text-field
                     label="Message:"
                     v-model="message"
                     hide-details="auto"
                   ></v-text-field>
                 <v-btn
                   text
                   color="primary"
                   :disabled="message === ''"
                   @click='sendMessage'
                 >
                    Send Message
                 </v-btn>
               </v-card-text> 
             </v-card>   
         </div>
    </div>
</template>

<script> 
    import SocketService from '../services/SocketService.ts';
    export default {
      name: 'Feed',
      data: () => ({
        message: "",
        socket: new SocketService(),
      }),
      methods: {
          feed() {
            console.log(this.$feed);
            return this.$feed;   
          },
          sendMessage() {
            this.$wsService.send_message(this.message);
            this.message = "";
            return;
          }
        }
    };
</script>
