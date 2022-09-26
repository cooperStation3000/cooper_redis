<template>
  <n-card style="width: 600px" title="模态框" :bordered="false" size="huge" role="dialog" aria-modal="true">
    <template #header-extra>新建连接</template>
    new link
    <template #footer>
      <NSpace>
        <n-button type="info" strong size="large" @click="cancel">取消</n-button>
        <n-button type="info" strong size="large" @click="ok">确定</n-button>
      </NSpace>
    </template>
  </n-card>
</template>
<script lang="ts" setup>
import { NCard, NSpace, NButton } from 'naive-ui';

function cancel() {}
function ok() {
  
}
</script>
<style scoped lang="less"></style>
