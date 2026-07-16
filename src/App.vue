<script setup lang="ts">
import { mdiFormatListBulleted, mdiCalendarBlankOutline, mdiCogOutline } from '@mdi/js'
import { computed, ref } from 'vue'
const value = ref(0)
const color = computed(() => {
  switch (value.value) {
    case 0: return 'indigo'
    case 1: return 'teal'
    case 2: return 'brown'
    default: return 'blue-grey'
  }
})
function updateValue(this: any, newValue: number) {
  value.value = newValue
  if(value.value != 0 && value.value != 1 && value.value != 2) {
    this.$router.push('/')
  }
}
</script>

<template>
  <v-app class="container overflow-visible" style="height: 56px">
    <v-main>
      <RouterView style="height: 100%;" />
    </v-main>
    <v-bottom-navigation v-model="value" @update:model-value="updateValue" :bg-color="color" mode="shift">
      <v-btn @click="$router.push('/set')">
        <v-icon :icon="mdiFormatListBulleted"></v-icon>
        <span>便签集</span>
      </v-btn>
      <v-btn @click="$router.push('/board')">
        <v-icon :icon="mdiCalendarBlankOutline"></v-icon>
        <span>记事板</span>
      </v-btn>
      <v-btn @click="$router.push('/settings/introduce')">
        <v-icon :icon="mdiCogOutline"></v-icon>
        <span>设置</span>
      </v-btn>
    </v-bottom-navigation>
  </v-app>
</template>
<style>

</style>
