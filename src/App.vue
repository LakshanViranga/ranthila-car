<template>
  <v-app>
    <v-main>
      <Login v-if="!isLoggedIn" @login="handleLogin"/>
      <v-container v-else fluid>
        <v-row>
          <v-col cols="12">
            <router-view @logout="handleLogout"/>
            <Toast/>
          </v-col>
        </v-row>
      </v-container>
    </v-main>

  </v-app>
</template>

<script>
import Login from './component/Login.vue';
import { useProductStore } from './stores/product.ts';
import { useAuthStore } from "./stores/auth.ts";
import Toast from "./component/Toast.vue";
import {dbService} from "./services/db.ts";
import { useSnackbar } from './composables/useSnackbar'
const { showSuccess, showError } = useSnackbar()

export default {
  components: {
    Toast,
    Login
  },
  data() {
    return {
      isLoggedIn: false,
    };
  },
  methods: {
    async handleLogin(username, password) {
      try {
        const response = await dbService.login({
          username, password
        });
        if (response.success) {
          this.isLoggedIn = true;
          const authStore = useAuthStore();
          authStore.setAuth(response.user.username, response.user.role);
        } else {
         showError(response.message);
        }
      }catch(err) {
        showError(err);
        console.log(err);
      }
    },
    handleLogout() {
      this.isLoggedIn = false;
      const authStore = useAuthStore();
      authStore.resetAuth();
    }
  },
  async mounted() {
    const productStore = useProductStore();
    await productStore.fetchProducts();
  }
};
</script>
