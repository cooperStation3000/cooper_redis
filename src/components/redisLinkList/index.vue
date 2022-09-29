<template>
  <n-collapse arrow-placement="right">
    <n-collapse-item
      v-for="ele in links"
      :title="ele.alias === '' ?  `${ele.ip}@${ele.port}` : ele.alias "
      :name="ele.ip"
      :key="ele.ip"
    >
    <template #arrow>
      <div></div>
      <!-- <n-icon>
        <cash-icon />
      </n-icon> -->
    </template>
    <template #header-extra>
      <!-- TODO: -->
      </template>

      <!-- <div>{{ ele.ip }}@{{ ele.port }}</div> -->
    </n-collapse-item>
  </n-collapse>
</template>
<script lang="ts" setup>
import { RedisLinkInfo } from '@/types';
import { NCollapse, NCollapseItem } from 'naive-ui';
import { computed } from 'vue';
import { useStore } from 'vuex';
const links = computed<RedisLinkInfo[]>(() => useStore().state.links);
</script>
<style scoped lang="less"></style>
