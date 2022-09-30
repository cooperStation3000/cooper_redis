<template>
  <n-collapse arrow-placement="right">
    <n-collapse-item
        v-for="(ele, index) in links"
        :title="ele.alias === '' ? `${ele.ip}@${ele.port}` : ele.alias"
        :name="ele.ip"
        :key="`${index}_${ele.ip}@${ele.port}`"
    >
      <template #header-extra>
        <NSpace>
          <n-button quaternary type="info" size="tiny" @click.stop.prevent="showHome">
            <template #icon>
              <n-icon>
                <Home12Regular/>
              </n-icon>
            </template>
          </n-button>
          <n-button quaternary type="info" size="tiny" @click.stop.prevent="reflash">
            <template #icon>
              <n-icon>
                <ArrowCounterclockwise16Regular/>
              </n-icon>
            </template>
          </n-button>
          <NPopselect :value="''" :options="settings" @update:value="doCmd">
            <n-button quaternary type="info" size="tiny" @click.stop.prevent="">
              <template #icon>
                <n-icon>
                  <Settings20Regular/>
                </n-icon>
              </template>
            </n-button>
          </NPopselect>
        </NSpace>
      </template>

      <div class="content">
        <LinkItem/>
      </div>
    </n-collapse-item>
  </n-collapse>
</template>
<script lang="ts" setup>
import {RedisLinkInfo} from '@/types';
import {NButton, NCollapse, NCollapseItem, NIcon, NPopselect, NSpace} from 'naive-ui';
import {ArrowCounterclockwise16Regular, Home12Regular, Settings20Regular} from '@vicons/fluent';
import LinkItem from '@/components/linkItem/linkItem.vue'
import {computed} from 'vue';
import {useStore} from 'vuex';
import {settings} from './data';

const links = computed<RedisLinkInfo[]>(() => useStore().state.links);

function showHome() {
  console.log('12312');
}

function reflash() {
  console.log('12312');
}

async function doCmd(value: string) {
  console.log('[ value ] >', value);
}
</script>
<style scoped lang="less"></style>
