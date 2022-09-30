<template>
  <div class="btns">
    <n-grid x-gap="6" :cols="12">
      <NGridItem :span="6">
        <n-select v-model:value="db" :options="dbArr"/>
      </NGridItem>
      <NGridItem :span="6">
        <NButton>+ 新增Key</NButton>
      </NGridItem>
      <NGridItem :span="12">
        <NInput v-model:value="selectKey" placeholder="Enter 搜索" clearable @keydown.enter="search">
          <template #suffix>
            <n-tooltip trigger="hover" placement="bottom">
              <template #trigger>
                <n-checkbox v-model:checked="isAccurateSearch"/>
              </template>
              精确搜索
            </n-tooltip>
          </template>
        </NInput>
      </NGridItem>
    </n-grid>
    <div class="keyList">
      <n-list v-if="keyList.length">
        <n-list-item v-for="key in keyList">{{ key }}</n-list-item>
      </n-list>
      <p v-else>
        暂无数据
      </p>
    </div>

    <div class="footer">
      <NSpace justify="space-between">
        <NButton :disabled="!keyList.length" @click="loadMore">加载更多</NButton>
        <NButton type="error" @click="loadAll">加载所有</NButton>
      </NSpace>
    </div>
  </div>


</template>
<script lang="ts" setup>
import {NButton, NCheckbox, NGrid, NGridItem, NInput, NList, NListItem, NSelect, NSpace, NTooltip} from 'naive-ui'
import {ref} from 'vue'

const db = ref(0);
//TODO:将key字段和实际的 redis key 联系起来
const dbArr = ref(new Array(16).fill(0).map((ele, index) => ({label: `DB${index}`, value: index})))
const selectKey = ref('')
const isAccurateSearch = ref(false)
const keyList = ref([])

function search() {
  console.log('[ search ] ');
}

function loadMore() {
}

function loadAll() {
}

</script>

<style scoped lang="less">
</style>
