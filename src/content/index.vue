<template>
  <div class="content">
    <NTabs v-model:value="name" type="card" closable tab-style="min-width: 80px;" @close="handleClose">
      <NTabPane v-for="panel in panels" :key="panel" :tab="panel.toString()" :name="panel">
        {{ panel }}
      </NTabPane>
    </NTabs>
    <!-- <NButton @click="sendMsg">发送指令</NButton> -->
  </div>
</template>

<script lang="ts" setup>
import { NButton, NTabs, NTabPane } from 'naive-ui';
import { ref } from 'vue';
const name = ref('tets');
const panels = ref([1, 2, 3]);
// import { invoke } from '@tauri-apps/api';

function handleClose(name: number) {
  console.log('%c [ name ]-28-「index.vue」', 'font-size:13px; background:pink; color:#bf2c9f;', name);
}
// async function sendMsg() {
//   const res = await invoke('get_redis_info', { tet: '12312' });
//   alert(res)
// }
</script>
