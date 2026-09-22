<template>
  <div class="login-container">
    <div class="login-card">

      <!-- Row 1: Logo -->
      <div class="logo-section">
        <div class="logo-circle">
          <img src="../../images/ranthila.jpeg" alt="logo" />
        </div>
      </div>

      <!-- Row 2: Restaurant Name -->
      <div class="name-section">
        <h2>Ranthila Rent A Car</h2>
      </div>

      <!-- Login Form -->
      <form class="login-form" @submit.prevent="handleLogin">

        <!-- Username -->
        <div class="input-group">
          <label for="username">Username</label>
          <div class="input-wrapper">
            <i class="fa fa-user"></i>
            <input
                id="username"
                v-model="username"
                type="text"
                placeholder="Enter username"
                autocomplete="username"
                required
            />
          </div>
        </div>

        <!-- Password -->
        <div class="input-group">
          <label for="password">Password</label>
          <div class="input-wrapper">
            <i class="fa fa-lock"></i>
            <input
                id="password"
                v-model="password"
                type="password"
                placeholder="Enter password"
                autocomplete="current-password"
                required
            />
          </div>
        </div>

        <!-- Login Button -->
        <div class="button-section">
          <button class="enter-button" type="submit">
            <i class="fa fa-sign-in"></i>
            Enter
          </button>
        </div>

        <!-- Error Message -->
        <div v-if="errorMessage" class="error-message">
          <i class="fa fa-exclamation-circle"></i>
          {{ errorMessage }}
        </div>

      </form>
    </div>
  </div>
</template>

<script>
import { dbService } from '../services/db.ts';

export default {
  name: 'Login',

  data() {
    return {
      username: '',
      password: '',
      errorMessage: ''
    };
  },

  methods: {
    async handleLogin() {
      this.errorMessage = '';

      if (!this.username.trim()) {
        this.errorMessage = 'Please enter your username.';
        return;
      }

      if (!this.password) {
        this.errorMessage = 'Please enter your password.';
        return;
      }
      this.$emit('login', this.username, this.password);
    }
  }
};
</script>

<style scoped>
.login-container {
  width: 100%;
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(
      135deg,
      #667eea 0%,
      #764ba2 25%,
      #f093fb 50%,
      #4facfe 75%,
      #00f2fe 100%
  );
  background-size: 400% 400%;
  animation: gradient 15s ease infinite;
  font-family: serif;
}

@keyframes gradient {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

.login-card {
  background: cornsilk;
  border-radius: 3px;
  padding: 3rem 3rem;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.logo-section {
  display: flex;
  justify-content: center;
  padding: 0.5rem 0;
}

.logo-circle {
  width: 100px;
  height: 100px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  box-shadow: 0 5px 20px rgba(102, 126, 234, 0.25);
}

.logo-circle img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.name-section {
  display: flex;
  justify-content: center;
  text-align: center;
}

.name-section h2 {
  margin: 0;
  font-size: 28px;
  font-weight: 500;
  color: #71ae19;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.input-group label {
  font-size: 15px;
  font-weight: 600;
  color: #333;
}

.input-wrapper {
  position: relative;
  width: 100%;
}

.input-wrapper i {
  position: absolute;
  left: 14px;
  top: 50%;
  transform: translateY(-50%);
  color: #777;
  font-size: 15px;
}

.input-wrapper input {
  width: 100%;
  box-sizing: border-box;
  padding: 12px 14px 12px 42px;
  font-family: serif;
  font-size: 15px;
  color: #333;
  background: white;
  border: 1px solid #d8d8d8;
  border-radius: 3px;
  outline: none;
  transition: all 0.25s ease;
}

.input-wrapper input::placeholder {
  color: #999;
}

.input-wrapper input:focus {
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.12);
}

.button-section {
  display: flex;
  justify-content: center;
  padding: 0.5rem 0 0;
}

.enter-button {
  width: 100%;
  padding: 12px 48px;
  font-family: serif;
  font-size: 16px;
  font-weight: 500;
  border: none;
  border-radius: 3px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
}

.enter-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
}

.enter-button:active {
  transform: translateY(0);
}

.enter-button i {
  margin-right: 8px;
}

.error-message {
  padding: 10px 12px;
  border-radius: 3px;
  background: #fff0f0;
  border: 1px solid #f3b4b4;
  color: #c62828;
  font-size: 13px;
  text-align: center;
}

@media (max-width: 500px) {
  .login-card {
    width: calc(100% - 32px);
    padding: 2.5rem 1.5rem;
  }

  .name-section h2 {
    font-size: 24px;
  }
}
</style>

