export type RedisLinkInfo = {
  ip: string;
  port: number;
  username?: string;
  password?: string;
  alias?: string;
  url?: string;
};

export type TabsInfo = {
  title: string;
  type: 'base' | 'info'; // base 为 当前链接的基本信息，info 则为 k/v 的信息
  linkInfo: RedisLinkInfo;
};
