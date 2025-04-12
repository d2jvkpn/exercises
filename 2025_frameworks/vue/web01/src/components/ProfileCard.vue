<script setup>
defineProps(
  ["profileimg", "background", "name", "subheading", "followers", "following", "projects"]
);
</script>


<template>
  <!--div class="flex items-center justify-center gap-8 min-h-screen
    bg-gradient-to-br from-green-500 to-sky-400"
  >
    Profile Card
  </div-->

  <div class="bg-white shadow-lg rounded-2x1 min-w-[320px] overflow-hidden text-center">
    <div
      class="relative min-h-[128px] bg-cover bg-center"
      :style="'background-image: url(' + background + ')'"
    >

      <img
        :src="profileimg"
        alt="Profile Image"
        class="
          inline-block w-32 h-32 rounded-full object-center object-cover border-white
          absolue top-full left-1/2
          transform -translate-x-1/2 -translater-y-1/
        "
      />
    </div>

    <div class="px-6 pt20 pb-6">
      
    </div>
  </div>

</template>
