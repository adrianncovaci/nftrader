<template>
 <v-card
    class="mx-auto"
  >
    <v-card-text>
        <v-text-field
          label="Nickname:"
          v-model="nickname"
          :rules="rules"
          hide-details="auto"
        ></v-text-field>
    </v-card-text> 
    <v-card-actions>
      <v-btn
        text
        color="primary"
        @click='login'
      >
        Login
      </v-btn>
    </v-card-actions>
  </v-card>   
</template>

<script> 
    import SocketService from '../services/SocketService';
    export default {
      name: 'Login',

      data: () => ({
        rules: [
            rule => !!rule || "Please provide a nickname",
        ],
        nickname: '',
        socket: new SocketService()
      }),
      methods: {
          login() {
            this.socket.login(this.nickname);
          }
      }
    };
</script>
