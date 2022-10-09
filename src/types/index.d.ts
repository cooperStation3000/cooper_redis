import {TabsInfoType} from "@/util/dist";

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
    type: TabsInfoType;
    linkInfo: RedisLinkInfo;
};
