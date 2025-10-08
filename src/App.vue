<script setup lang="ts">
import Footer from './components/Footer.vue'
import {useRoute} from "vue-router";

const route = useRoute()
</script>

<template>
  <Suspense>
    <div class="h-screen w-screen flex flex-col">
      <div class="w-full flex-1 overflow-hidden">
        <router-view/>
      </div>
      <div v-if="!route.path.startsWith('/config') && !route.path.startsWith('/update')" class="w-full h-[31px]">
        <!--      <div class="w-full h-[31px]">-->
        <Footer/>
      </div>
    </div>
  </Suspense>
</template>
