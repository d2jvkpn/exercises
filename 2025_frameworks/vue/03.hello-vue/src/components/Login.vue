<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

let metaEnv = import.meta.env;

console.log(`==> Login page: ${JSON.stringify(metaEnv)}`);

const loginForm = ref({ account: '', password: ''});
const errorMsg = ref('');
const isSubmitting = ref(false);

const submitLogin = async () => {
  if (!loginForm.value.account || !loginForm.value.password) {
    errorMsg.value = "Please enter account and password";
    return;
  }
  isSubmitting.value = true;
  errorMsg.value = "";

  console.log(`--> account login: ${JSON.stringify(loginForm.value)}`);

  try {
    // await new Promise(resolve => setTimeout(resolve, 1000));
    // throw new Error('This is a test error.');
    const response = await fetch(
      metaEnv.VITE_API_URL + "/login",
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(loginForm.value),
      }
    );

    if (!response.ok) {
      const responseErr = await response.json();
      throw new Error(responseErr.msg || 'Login Failed');
      return;
    }

    const responsData = await response.json();
    console.log('--> Login success', responsData);

    // localStorage.setItem('token', data.token);
    router.push('/dashboard')
  } catch (err) {
    console.log(`!!! ${err}`);
    errorMsg.value = err.message || 'Login Failed';
  } finally {
    isSubmitting.value = false;
  }
}
</script>


<template>
<div class="login-page bg-gradient-to-br from-green-500 to-sky-400">
  <form class="login-form" @submit.prevent="submitLogin">
    <span> Account </span>
    <input
      type="text" id="account" name="account" required minlength="6" maxlength="32" size="10"
      placeholder="phone or email" v-model="loginForm.account"
    />

    <span> Password </span>
    <input
      type="password" id="password" name="password" required minlength="8" maxlength="32" size="10"
      placeholder="password" v-model="loginForm.password"
    />

    <p v-if="errorMsg" class="login-error-msg">
     {{ errorMsg }}
    </p>

    <button type="submit" :disabled="isSubmitting">
      {{ isSubmitting ? 'Logging...' : 'Login' }}
    </button>
  </form>
</div>
</template>


<style>
.login-page {
  height: 100vh;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  background-color: white;
  min-width: 20rem;
}

.login-form {
  border: 1px solid black;
  border-radius: 5px;
  padding: 2rem 2rem;
  margin: 1rem 1rem;
  display: grid;
  row-gap: 1rem;
  column-gap: 0.5rem;
  grid-template-columns: 1fr 3fr;
  /*justify-content: center;*/
  align-items: center;
  font-size: 1.2rem;
}

.login-form > span {
  text-align: right;
}

.login-form > input {
  border: 1px solid blue;
  padding: 0.5rem;
  min-width: 10rem;
}

.login-error-msg {
  font-size: 1rem;
  color: tomato;
  grid-column: 1 / span 2;
}

.login-form > button {
  background-color: green;
  border-radius: 5px;
  padding: 0.2rem;
  font-size: 1.5rem;
  color: white;
  grid-column: 1 / span 2;
}

@media (max-width: 720px) {
  .login-page {
    justify-content: center;
  }
}
</style>
