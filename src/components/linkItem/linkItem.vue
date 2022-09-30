<template>
  <div class="btns">
    <n-grid x-gap="6" :cols="12">
      <NGridItem :span="6">
        <n-select v-model:value="db" :options="dbArr"/>
      </NGridItem>
      <NGridItem :span="6">
        <NButton >+ 新增Key</NButton>
      </NGridItem>
      <NGridItem :span="12">
        <NInput/>
      </NGridItem>
    </n-grid>
  </div> 


</template>
<script lang="ts" setup>
import {NGrid, NGridItem, NSelect, NButton, NInput} from 'naive-ui'
import {ref} from 'vue'

const db = ref(0);
//TODO:将key字段和实际的 redis key 联系起来
const dbArr = ref(new Array(16).fill(0).map((ele, index) => ({label: `DB${index}`, value: index})))

</script>

<style scoped lang="less">
</style>
