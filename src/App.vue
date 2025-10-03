<script setup lang="ts">
import Footer from './components/Footer.vue'
</script>

<template>
  <Suspense>
    <div class="h-screen w-screen flex flex-col">
      <div class="w-full flex-1 overflow-hidden">
        <router-view/>
      </div>
      <div class="w-full h-[31px]">
        <Footer/>
      </div>
    </div>
  </Suspense>
</template>
