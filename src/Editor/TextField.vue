<script setup lang="ts">
import VueDragResizeRotate from '@liaogn/vue-drag-resize-rotate'
import '@liaogn/vue-drag-resize-rotate/style.css'

const blocks = ref([
  { id: 1, x: 50, y: 50, width: 100, height: 100, rotation: 0 },
  { id: 2, x: 200, y: 200, width: 150, height: 150, rotation: 0 },
])
</script>
<template>
  <v-sheet>
    <v-sheet v-for="block in blocks" :key="block.id" :style="{ position: 'absolute', left: block.x + 'px', top: block.y + 'px', width: block.width + 'px', height: block.height + 'px', transform: 'rotate(' + block.rotation + 'deg)' }">
      <VueDragResizeRotate
        v-model:x="block.x"
        v-model:y="block.y"
        v-model:width="block.width"
        v-model:height="block.height"
        v-model:rotation="block.rotation"
        :min-width="50"
        :min-height="50">
      
      </VueDragResizeRotate>
    </v-sheet>
  </v-sheet>
</template>