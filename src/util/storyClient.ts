import {RedisLinkInfo} from '@/types';
import {useStore} from 'vuex'

enum KEYS {
    LINK = 'link'
}

export default class Storage {
    public static getLinkList(): RedisLinkInfo[] {
        const data = localStorage.getItem(KEYS.LINK);
        if (data) return JSON.parse(data);
        else return [];
    }

    public static setLinkItem(link: RedisLinkInfo) {
        const old = Storage.getLinkList();
        old.push(link);
        (async () => {
            await useStore().dispatch('setLink', link)
        })()
        localStorage.setItem(KEYS.LINK, JSON.stringify(old));

    }
}
