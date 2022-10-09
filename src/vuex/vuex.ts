import {createStore} from 'vuex';

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
        },
        pushTab(state, data) {
            state.tabs = data
        }
    },
    actions: {
        setLink({commit}, data) {
            commit('setLink', data);
        },
        pushTab({commit}, data) {
            commit('pushTab', data);
        }
    },
    modules: {}
})
