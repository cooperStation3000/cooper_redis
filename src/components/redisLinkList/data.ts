import { KEYS } from './../../util/dist';
import { reactive } from 'vue';

export const settings = reactive([
  {
    label: '关闭连接',
    value: KEYS.close_connect
  },
  {
    label: '编辑连接',
    value: KEYS.edit_connect
  },
  {
    label: '删除连接',
    value: KEYS.delete_connect
  },
  {
    label: '复制连接',
    value: KEYS.copy_connect
  },
  {
    label: '删除所有key',
    value: KEYS.delete_all_keys
  }
]);
