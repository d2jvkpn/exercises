<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Fold, Expand, ArrowDown } from '@element-plus/icons-vue'

import Sidebar from "./Sidebar.vue"
import HeaderBar from "./HeaderBar.vue"

const username = ref('Jane')
const isHidden = ref(false)

const toggleCollapse = () => {
  isHidden.value = !isHidden.value
}

const router = useRouter()

const handleCommand = (command) => {
  switch (command) {
    case 'profile':
      router.push('/settings/profile')
      break
    case 'security':
      router.push('/settings/security')
      break
    case 'logout':
      localStorage.removeItem('token')
      router.push('/login')
      break
  }
}
</script>

<template>
<div class="overview">
  <HeaderBar :username="username" @toggleSidebar="isHidden = !isHidden" />

  <div class="overview-main">
    <Sidebar v-show="!isHidden" />

    <main class="overview-content">
      <router-view v-slot="{ Component }">
        <KeepAlive>
          <component :is="Component" />
        </KeepAlive>
      </router-view>
    </main>
  </div>
</div>
</template>

<style>
.overview {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.overview-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.overview-content {
  position: relative;
  flex: 1;
  padding: 20px;
  background-color: #f5f7fa;
  overflow-y: auto;
}
</style>
