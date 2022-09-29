<template>
  <n-card style="width: 620px" title="新建连接" :bordered="false" size="huge" role="dialog" aria-modal="true">
    <n-switch v-model:value="modeFlag">
      <template #checked>连接模式</template>
      <template #unchecked>默认模式</template>
    </n-switch>

    <NForm :rules="role" :model="linkInfo" inline :label-width="200" ref="formRef">
      <n-grid :cols="24" :x-gap="24" v-if="modeFlag">
        <NFormItemGi :span="24" label="连接url" path="url">
          <NInput v-model:value="linkInfo.url" placeholder="redis://username:password@host:port"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="连接别名">
          <NInput v-model:value="linkInfo.alias"></NInput>
        </NFormItemGi>
      </n-grid>
      <n-grid :cols="24" :x-gap="24" v-else>
        <NFormItemGi :span="12" label="地址" path="ip">
          <NInput v-model:value.number="linkInfo.ip" placeholder="127.0.0.1"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="端口" path="port">
          <NInput v-model:value="linkInfo.port" placeholder="6379" maxlength="4"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="用户名">
          <NInput v-model:value="linkInfo.username"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="密码">
          <NInput v-model:value="linkInfo.password" type="password"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="连接别名">
          <NInput v-model:value="linkInfo.alias"></NInput>
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
import {
  FormInst,
  FormItemRule,
  FormRules,
  NButton,
  NCard,
  NForm,
  NFormItemGi,
  NGrid,
  NInput,
  NSpace,
  NSwitch,
  useMessage
} from 'naive-ui';
import { reactive, ref } from 'vue';
import { RedisLinkInfo } from '@/types';
import _ from 'lodash';

const formRef = ref<FormInst | null>(null);
defineProps({
  value: Boolean
});

const emit = defineEmits(['change']);

const linkInfo = reactive<RedisLinkInfo>({
  ip: '',
  port: '6379' as unknown as number,
  username: '',
  password: '',
  alias: '',
  url: ''
});
const role = reactive<FormRules>({
  ip: [
    {
      required: true,
      message: '请输入正确的ip',
      validator(rule: FormItemRule, value: string) {
        const reg =
          /^(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|[1-9])\.(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|\d)\.(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|\d)\.(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|\d)$/;
        return reg.test(value);
      },
      trigger: ['input', 'blur']
    }
  ],
  port: [
    {
      required: true,
      message: '端口错误',
      validator(rule: FormItemRule, value: string) {
        const reg = /^\d{1,4}$/gi;
        return !(!reg.test(value) || Number(value) < 0);
      },
      trigger: ['input', 'blur']
    }
  ],
  url: [
    {
      required: true,
      message: '连接不正确',
      validator(rule: FormItemRule, value: string) {
        const reg = /^redis:\/\/(\w{1,}:\w{1,}@)?(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,4})$/gi;
        return reg.test(value);
      },
      trigger: ['input', 'blur']
    }
  ]
});
const modeFlag = ref(false);

function cancel() {
  emit('change', false);
}

function ok() {
  // emit('change', false);
  formRef.value?.validate(err => {
    if (err) {
      useMessage().error('校验不通过!');
      return;
    }
    const parms = modeFlag ? ['url'] : ['ip', 'port', 'username', 'password'];
    const comm = ['alias'];
    const data = _.pick(linkInfo, parms, comm);

  });
}
</script>
<style scoped lang="less"></style>
