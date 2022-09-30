import {RedisLinkInfo} from '@/types';
import Stroy from '@/vuex/vuex'

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
        //@ts-ignore
        Stroy.state.links = old
        localStorage.setItem(KEYS.LINK, JSON.stringify(old));

    }
}
