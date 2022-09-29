import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import 'normalizecss/normalize.css'
// 通用字体
// import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'
import store from '@/vuex/vuex'
import Storage from "@/util/storyClient";

createApp(App).use(store).mount("#app")

// init-link

store.dispatch('setLink', Storage.getLinkList())


