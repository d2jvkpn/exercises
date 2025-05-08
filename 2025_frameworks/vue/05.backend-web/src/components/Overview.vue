<script setup>
import { ref } from 'vue'
import { Fold, Expand } from '@element-plus/icons-vue'

import SidebarMenu from "./SidebarMenu.vue"


const username = ref('Leonard')
const isCollapsed = ref(false)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
}
</script>

<template>
<div class="admin-layout">
  <header class="header-bar">
    <div class="logo"> 🌿 Welcome </div>
    <div class="user-info"> {{username}} </div>
  </header>

  <div class="main-area">
    <SidebarMenu v-show="!isCollapsed" />

      <main class="content">
        <div class="menu-toggle-btn">
          <el-button text circle @click="toggleCollapse">
            <el-icon>
              <component :is="isCollapsed ? Expand : Fold" />
            </el-icon>
          </el-button>
        </div>

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
.admin-layout {
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

.main-area {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.menu-toggle-btn {
  position: absolute;
  top: 2px;
  left: 2px;
  z-index: 10;
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
