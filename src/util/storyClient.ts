import { RedisLinkInfo } from '@/types';

enum KEYS {
  LINK = 'link'
}

export default class Storyge {
  public static getLinkList(): RedisLinkInfo[] {
    const data = localStorage.getItem(KEYS.LINK);
    if (data) return JSON.parse(data);
    else return [];
  }
  public static setLinkItem(link: RedisLinkInfo) {
    const old = Storyge.getLinkList();
    old.push(link);
    localStorage.setItem(KEYS.LINK, JSON.stringify(old));
  }
}
