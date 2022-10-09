import { createStore } from 'vuex';

export default createStore({
  state: {
    links: [],
    tabs: []
  },
  getters: {
    getLinkList: state => state.links,
    getTabs: state => state.tabs
  },
  mutations: {
    setLink(state, data) {
      state.links = data;
    }
  },
  actions: {
    setLink({ commit }, data) {
      commit('setLink', data);
    }
  },
  modules: {}
});
