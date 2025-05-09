<script setup>
import { useRouter } from 'vue-router'
import { ArrowDown, Fold, Expand } from '@element-plus/icons-vue'


//
const props = defineProps({
  accountName: String,
  isSidebarHidden: Boolean,
})

const emit = defineEmits(['toggleSidebar'])

//
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
<header class="headerbar">
  <div class="headerbar-left">
    <el-button text circle @click="$emit('toggleSidebar')" class="headerbar-toggle-btn">
      <el-icon>
        <component :is="isSidebarHidden ? Expand : Fold" />
       </el-icon>
    </el-button>
    <div class="headerbar-logo"> 🌿 Welcome </div>
  </div>

  <el-dropdown @command="handleCommand">
    <span class="headerbar-account el-dropdown-link">
      {{ accountName }}
      <el-icon> <ArrowDown /> </el-icon>
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
</template>

<style scoped>
.headerbar {
  height: 60px;
  background-color: #fff;
  padding: 0 20px;
  font-size: 16px;
  border-bottom: 1px solid #eee;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.headerbar-left {
  display: flex;
  align-items: center;
  gap: 0.2rem;
}

.headerbar-toggle-btn {
  font-size: 1.2rem;
  /*
  position: absolute;
  top: 2px;
  left: 2px;
  z-index: 10;
*/
}

.headerbar-logo {
  font-size: 18px;
  color: #409eff;
  font-weight: bold;
}

.headerbar-account {
  cursor: pointer;
  font-size: 14px;
  color: #333;
  display: flex;
  align-items: center;
  gap: 4px;
}

.headerbar-account.el-dropdown-link {
  transition: font-size 0.2s ease;
  border: none !important;
}

.headerbar-account.el-dropdown-link:hover {
  border: none !important;
  background: none !important;
  font-size: 16px;
  color: #409eff;
}
</style>
