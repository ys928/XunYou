import { createMemoryHistory, createRouter } from 'vue-router'
import LeftSidbarView from '../views/LeftSidebarView.vue'
import RightSidbarView from '../views/RightSidbarView.vue'
import CenterView from '../views/CenterView.vue'

const router = createRouter({
    history: createMemoryHistory(),
    routes: [
        {
            path: '/',
            components: {
                default: CenterView,
                LeftSidebar: LeftSidbarView,
                RightSidebar: RightSidbarView,
            }
        }
    ],
})

export default router;