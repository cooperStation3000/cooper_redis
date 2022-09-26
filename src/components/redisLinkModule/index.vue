<template>
  <n-card style="width: 620px" title="新建连接" :bordered="false" size="huge" role="dialog" aria-modal="true">
    <NForm :role="role" :model="linkInfo" inline :label-width="200" ref="formRef">
      <n-grid :cols="24" :x-gap="24">
        <NFormItemGi :span="12" label="地址" path="ip">
          <NInput v-model:value.number="linkInfo.ip" placeholder="127.0.0.1"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="端口" path="port">
          <NInput v-model="linkInfo.port" placeholder="6379"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="用户名" path="username">
          <NInput v-model:value="linkInfo.username"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="密码" path="password">
          <NInput v-model:value="linkInfo.password" type="password"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="24" label="连接名称" path="alias">
          <NInput v-model:value="linkInfo.alias" placeholder="redis://username:password@host:port"></NInput>
        </NFormItemGi>
      </n-grid>
    </NForm>

    <template #footer>
      <NSpace>
        <n-button type="info" strong size="large" @click="cancel">取消</n-button>
        <n-button type="info" strong size="large" @click="ok">确定</n-button>
      </NSpace>
    </template>
  </n-card>
</template>
<script lang="ts" setup>
import { NButton, NCard, NForm, NFormItemGi, NGrid, NInput, NSpace, FormInst, FormItemRule } from 'naive-ui';
import { reactive, ref } from 'vue';
import { RedisLinkInfo } from '@/types';
const formRef = ref<FormInst | null>(null);
const props = defineProps({
  value: Boolean
});

const emit = defineEmits(['change']);

const linkInfo = reactive<RedisLinkInfo>({
  ip: '',
  port: 6379,
  username: '',
  password: '',
  alias: ''
});
const role = reactive({
  ip: {
    required: true,
    message: '请输入ip',
    validator(rule: FormItemRule, value: string) {},
    trigger: ['input', 'blur']
  },
  port: {
    required: true,
    message: '请输入端口',
    trigger: 'blur'
  },
  username: {
    required: true,
    message: '请输入用户名',
    trigger: 'blur'
  },
  password: {
    required: true,
    message: '请输入密码',
    trigger: 'blur'
  }
});

function cancel() {
  emit('change', false);
}

function ok() {
  // emit('change', false);
}
</script>
<style scoped lang="less"></style>
