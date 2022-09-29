import { createStore } from 'vuex'

export default createStore({
    state: {
        links:[]
    },
    getters: {
        getLinkList: state => state.links
    },
    mutations: {
        setLink(state,data){
            state.links = data;
        }
    },
    actions: {
        setLink({commit}, data){
            commit ('setLink',data)
        }
    },
    modules: {
    }
})
