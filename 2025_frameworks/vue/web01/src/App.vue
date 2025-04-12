<script setup lang="ts">
import ProfileCard from "./components/ProfileCard.vue"
</script>

<template>
  <div
    class="
      flex items-center justify-center gap-8 min-h-screen bg-gradient-to-br from-green-500
      to-sky-400
    "
  >

    <ProfileCard
      background="/background.jpeg"
      profileimg="/profile-image.jpeg"
    />
  </div>

</template>
