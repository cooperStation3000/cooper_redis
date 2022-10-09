<template>
  <div class="content">
    <NButton @click="sendMsg">发送指令</NButton>
  </div>
</template>

<script lang="ts" setup>
import { NButton } from 'naive-ui';
import { invoke } from '@tauri-apps/api';

async function sendMsg() {
  const res = await invoke('get_redis_info', { tet: '12312' });
  alert(res)
}
</script>
