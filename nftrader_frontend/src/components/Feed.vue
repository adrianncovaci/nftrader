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
                <v-file-input
                  hide-input
                  show-size
                  truncate-length="15"
                  style="width:50px; padding-top: 0px;"
                  accept="image/*"
                  @change="uploadImage"
                ></v-file-input> 


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
          },
          async uploadImage(image) {
            const toBase64 = file => new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.readAsDataURL(file);
                reader.onload = () => resolve(reader.result);
                reader.onerror = error => reject(error);
            });
            const content = await toBase64(image);
            this.$wsService.send_picture(content);
          }
        }
    };
</script>
