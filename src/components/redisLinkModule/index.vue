<template>
  <n-card style="width: 620px" title="新建连接" :bordered="false" size="huge" role="dialog" aria-modal="true">
    <NForm :rules="role" :model="linkInfo" inline :label-width="200" ref="formRef">
      <n-grid :cols="24" :x-gap="24">
        <NFormItemGi :span="12" label="地址" path="ip">
          <NInput v-model:value.number="linkInfo.ip" placeholder="127.0.0.1"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="端口" path="port">
          <NInput v-model:value.number="linkInfo.port" placeholder="6379"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="用户名">
          <NInput v-model:value="linkInfo.username"></NInput>
        </NFormItemGi>
        <NFormItemGi :span="12" label="密码" >
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
import {NButton, NCard, NForm, NFormItemGi, NGrid, NInput, NSpace, FormInst, FormItemRule, FormRules, useMessage} from 'naive-ui';
import {reactive, ref} from 'vue';
import {RedisLinkInfo} from '@/types';
import _ from 'lodash';

const formRef = ref<FormInst | null>(null);
defineProps({
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
const role = ref<FormRules>({
  ip: [{
    required: true,
    message: '请输入正确的ip',
    validator(rule: FormItemRule, value: string) {
      const reg = /^(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|[1-9])\.(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|\d)\.(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|\d)\.(1\d{2}|2[0-4]\d|25[0-5]|[1-9]\d|\d)$/
      return  reg.test(value)
    },
    trigger: ['input', 'blur']
  }],
  port: [{
    required: true,
    message: '请输入端口',
    validator(rule: FormItemRule, value: string) {
      const reg = /^\d{4}$/ig
      if(!reg.test(value) || Number(value)<0) return false;
      return true
    },
    trigger:  ['input', 'blur']
  }]
});

const message = useMessage()

function cancel() {
  emit('change', false);
}

function ok() {
  // emit('change', false);
  formRef.value?.validate(err=>{
    if(err){
      message.error('校验不通过!')
      return;
    }
    console.log('[ linkInfo ] >',linkInfo )
  })


}
</script>
<style scoped lang="less"></style>
