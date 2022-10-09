<template>
  <div class="content">
    <NTabs v-model:value="name" type="card" closable tab-style="min-width: 80px;" @close="handleClose">
      <NTabPane v-for="(panel, index) in tabs" :key="panel" :tab="panel.title" :name="index">
        <base-content v-if="panel.type===0" />
        <info-content v-else />
      </NTabPane>
    </NTabs>
    <!-- <NButton @click="sendMsg">发送指令</NButton> -->
  </div>
</template>

<script lang="ts" setup>
import { NTabPane, NTabs } from 'naive-ui';
import { computed, ref } from 'vue';
import { TabsInfo } from '@/types';
import { useStore } from 'vuex';
import BaseContent from '@/components/paneContent/baseContent.vue';
import InfoContent from '@/components/paneContent/infoContent.vue';

const name = ref('tets');
const panels = ref([1, 2, 3]);
const store = useStore();
const tabs = computed<TabsInfo[]>(() => store.state.tabs);

// import { invoke } from '@tauri-apps/api';

function handleClose(index: number) {
  tabs.value.splice(index, 1);
}

// async function sendMsg() {
//   const res = await invoke('get_redis_info', { tet: '12312' });
//   alert(res)
// }
</script>
