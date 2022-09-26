<template>
  <NMessageProvider>

  <n-layout has-sider>
    <n-layout-sider bordered content-style="padding: 24px; height:100vh" collapse-mode="transform">
      <NSpace>
        <n-button type="info" strong size="large" @click="createLink">
          <template #icon>
            <n-icon>
              <AddOutline/>
            </n-icon>
          </template>
          创建链接
        </n-button>
        <n-button strong secondary size="large" @click="showSetting">
          <template #icon>
            <n-icon>
              <SettingsOutline/>
            </n-icon>
          </template>
        </n-button>
      </NSpace>
    </n-layout-sider>

    <n-layout>
      <n-layout-content content-style="padding: 24px;"> 平山道</n-layout-content>
    </n-layout>
  </n-layout>

  <NModal v-model:show="moduleStatus.showAddLink">
    <RedisLinkModule :value="moduleStatus.showAddLink" @change="moduleHandler"></RedisLinkModule>
  </NModal>

  <NModal v-model:show="moduleStatus.showSetting">
    <n-card style="width: 600px" title="模态框" :bordered="false" size="huge" role="dialog" aria-modal="true">
      <template #header-extra> 噢！</template>
      setting
      <template #footer> 尾部</template>
    </n-card>
  </NModal>
</NMessageProvider>
</template>

<script lang="ts" setup>
import {NButton, NCard, NIcon, NLayout, NLayoutContent, NLayoutSider, NModal, NSpace, NMessageProvider} from 'naive-ui';
import {AddOutline, SettingsOutline} from '@vicons/ionicons5';
import RedisLinkModule from './components/redisLinkModule/index.vue'
import {reactive} from 'vue';


const moduleStatus = reactive({
  showAddLink: false,
  showSetting: false,
  showOptLog: false
});

function createLink() {
  moduleStatus.showAddLink = true;
}

function showSetting() {
  moduleStatus.showSetting = true;
}

function moduleHandler(e: boolean) {
  moduleStatus.showAddLink = e
}

</script>

<style lang="less"></style>
