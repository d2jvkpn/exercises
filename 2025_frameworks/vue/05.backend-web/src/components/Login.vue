<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const username = ref('')
const password = ref('')
const router = useRouter()

const login = () => {
  if (username.value && password.value) {
    localStorage.setItem('token', `${username.value}:${password.value}`)

    router.push('/dashboard')
  } else {
    alert('Please enter acocunt and password!')
  }
}
</script>

<template>
<div class="login-page">
  <el-card>
    <template #header>Please Login</template>
    <el-form @submit.prevent="login">
      <el-form-item>
        <el-input v-model="username" placeholder="email or phone" />
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

<style>
.login-page {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

.el-card {
  width: 30rem;
  /* margin-bottom: 10rem; */
}

.login-button {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
