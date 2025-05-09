<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const account = ref('')
const password = ref('')
const router = useRouter()

const login = () => {
  if (account.value && password.value) {
    localStorage.setItem('token', `${account.value}:${password.value}`)
    localStorage.setItem('accountName', "Jane Doe")
    localStorage.setItem('roles', JSON.stringify(["admin"]))

    router.push('/dashboard')
  } else {
    alert('Please enter acocunt and password!')
  }
}

console.log(`==> import.meta.env: ${JSON.stringify(import.meta.env)}`);
</script>


<template>
<div class="login">
  <el-card class="login-card">
    <template #header>Please Login</template>
    <el-form @submit.prevent="login">
      <el-form-item>
        <el-input v-model="account" placeholder="email or phone" />
      </el-form-item>

      <el-form-item>
        <el-input type="password" v-model="password" placeholder="password" />
      </el-form-item>

      <div class="login-button">
        <el-button type="primary" @click="login">Login</el-button>
      </div>
    </el-form>
  </el-card>
</div>
</template>


<style scoped>
.login {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

.login-card {
  width: 30rem;
  /* margin-bottom: 10rem; */
}

.login-button {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
