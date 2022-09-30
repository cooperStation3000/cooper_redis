<template>
  <n-collapse arrow-placement="right">
    <n-collapse-item
      v-for="(ele, index) in links"
      :title="ele.alias === '' ? `${ele.ip}@${ele.port}` : ele.alias"
      :name="ele.ip"
      :key="`${index}_${ele.ip}@${ele.port}`"
    >
      <!-- <template #arrow>
      <div></div>

    </template> -->
      <template #header-extra>
        <NSpace>
          <n-button quaternary type="info" size="tiny" @click.stop.prevent="handler">
            <template #icon>
              <n-icon><Home12Regular /></n-icon>
            </template>
          </n-button>
          <n-button quaternary type="info" size="tiny" @click.stop.prevent="handler">
            <template #icon>
              <n-icon>
                <ArrowCounterclockwise16Regular />
              </n-icon>
            </template>
          </n-button>
          <n-button quaternary type="info" size="tiny" @click.stop.prevent="handler">
            <template #icon>
              <n-icon>
                <Settings20Regular />
              </n-icon>
            </template>
          </n-button>
        </NSpace>
      </template>

      <div class="content"> asdfasdfasd </div>
    </n-collapse-item>
  </n-collapse>
</template>
<script lang="ts" setup>
import { RedisLinkInfo } from '@/types';
import { NCollapse, NCollapseItem, NIcon, NSpace, NButton } from 'naive-ui';
import { Home12Regular, ArrowCounterclockwise16Regular, Settings20Regular } from '@vicons/fluent';
import { computed } from 'vue';
import { useStore } from 'vuex';

const links = computed<RedisLinkInfo[]>(() => useStore().state.links);

function handler() {
  console.log('12312');
}
</script>
<style scoped lang="less"></style>
