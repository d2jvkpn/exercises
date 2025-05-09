<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Fold, Expand, ArrowDown } from '@element-plus/icons-vue'

import SidebarMenu from "./SidebarMenu.vue"

const username = ref('Jane')
const isCollapsed = ref(false)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
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
<div class="overview-layout">
  <header class="header-bar">
    <div class="left-header">
      <el-button text circle @click="toggleCollapse">
        <el-icon>
          <component :is="isCollapsed ? Expand : Fold" />
         </el-icon>
      </el-button>
      <div class="logo"> 🌿 Welcome </div>
    </div>

    <!--div class="account-info"> {{username}} </div-->
    <el-dropdown @command="handleCommand">
      <span class="account-info el-dropdown-link">
        {{ username }}
        <el-icon><ArrowDown /></el-icon>
      </span>


      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item command="profile">👤 Profile </el-dropdown-item>
          <el-dropdown-item command="security">⚙️ Security </el-dropdown-item>
          <el-dropdown-item divided command="logout">🚪 Logout </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </header>

  <div class="main-area">
    <SidebarMenu v-show="!isCollapsed" />

    <main class="content">
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
.overview-layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header-bar {
  height: 60px;
  background-color: #fff;
  padding: 0 20px;
  font-size: 16px;
  border-bottom: 1px solid #eee;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.left-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.menu-toggle-btn {
  padding: 4px;
  /*
  position: absolute;
  top: 2px;
  left: 2px;
  z-index: 10;
  */
}

.logo {
  font-size: 18px;
  color: #409eff;
  font-weight: bold;
}

.account-info {
  cursor: pointer;
  font-size: 14px;
  color: #333;
  display: flex;
  align-items: center;
  gap: 4px;
}

.account-info.el-dropdown-link {
  transition: font-size 0.2s ease;
  border: none !important;
}

.account-info.el-dropdown-link:hover {
  border: none !important;
  background: none !important;
  font-size: 16px;
  color: #409eff;
}

.main-area {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 200px;
  color: #fff;
  padding-top: 10px;
  overflow-y: auto;
}

.content {
  position: relative;
  flex: 1;
  padding: 20px;
  background-color: #f5f7fa;
  overflow-y: auto;
}
</style>
