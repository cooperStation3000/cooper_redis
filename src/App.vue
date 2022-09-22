<template>
  <n-layout has-sider>
    <n-layout-sider  bordered content-style="padding: 24px; height:100vh" collapse-mode="transform">
      <n-button type="info" size="large">
        <template #icon>
        <n-icon>
          <cash-icon />
        </n-icon>
      </template>
        创建链接</n-button>
    </n-layout-sider>
    <n-layout>
      <n-layout-content content-style="padding: 24px;">
        平山道
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import { NButton, NLayout, NLayoutSider,  NLayoutContent,NIcon } from 'naive-ui'
import { CashOutline as CashIcon } from '@vicons/ionicons5'

</script>

<style lang="less">




</style>