<script setup>
</script>

<template>
<aside class="sidebar">
  <el-menu :default-active="$route.path" router> <!-- :collapse="true" -->
    <el-menu-item index="/dashboard"> Dashboard </el-menu-item>
    <el-menu-item index="/accounts"> Accounts </el-menu-item>

    <el-sub-menu index="/settings">
      <template #title>Settings</template>
      <el-menu-item index="/settings/profile"> profile </el-menu-item>
      <el-menu-item index="/settings/security"> security </el-menu-item>
    </el-sub-menu>
  </el-menu>
</aside>
</template>
