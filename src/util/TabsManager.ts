import {RedisLinkInfo, TabsInfo,} from "@/types";
import {TabsInfoType} from "@/util/dist";

export default class TabsManager {

    public static genTab(info: RedisLinkInfo, type: TabsInfoType): TabsInfo {
        return {
            title: info.alias ?? `${info.ip}:${info.port}`,
            linkInfo: info,
            type
        }
    }

}
