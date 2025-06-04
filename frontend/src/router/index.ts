import { createRouter, createWebHistory, type RouteRecordInfo } from 'vue-router'
import HomeView from '../views/HomeView.vue'

export interface RouteNamedMap {
  TitleGroup: RouteRecordInfo<'TitleGroup', '/title-group/:id', { id: string | number }>
  Series: RouteRecordInfo<'Series', '/series/:id', { id: string | number }, { id: string }>
  Artist: RouteRecordInfo<'Artist', '/artist/:id', { id: string | number }, { id: string }>
}

declare module 'vue-router' {
  interface TypesConfig {
    RouteNamedMap: RouteNamedMap
  }
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'Login',
      component: () => import('../views/AuthView.vue'),
    },
    {
      path: '/register',
      name: 'Register',
      component: () => import('../views/AuthView.vue'),
    },
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/title-group/:id',
      name: 'TitleGroup',
      component: () => import('../views/TitleGroupView.vue'),
    },
    {
      path: '/torrents',
      name: 'Torrents',
      component: () => import('../views/TorrentSearchView.vue'),
    },
    {
      path: '/series/:id',
      name: 'Series',
      component: () => import('../views/SeriesView.vue'),
    },
    {
      path: '/artist/:id',
      name: 'Artist',
      component: () => import('../views/ArtistView.vue'),
    },
    {
      path: '/upload',
      name: 'UploadTorrent',
      component: () => import('../views/UploadTorrentView.vue'),
    },
    {
      path: '/user/:id',
      name: 'User',
      component: () => import('../views/UserView.vue'),
    },
    {
      path: '/forum',
      name: 'Forum',
      component: () => import('../views/forum/ForumOverviewView.vue'),
    },
    {
      path: '/forum/search',
      name: 'ForumSearch',
      component: () => import('../views/forum/ForumSearchView.vue'),
    },
    {
      path: '/forum/sub-category/:id',
      name: 'ForumSubCategory',
      component: () => import('../views/forum/ForumSubCategoryView.vue'),
    },
    {
      path: '/forum/thread/:id',
      name: 'ForumThread',
      component: () => import('../views/forum/ForumThreadView.vue'),
    },
    {
      path: '/forum/thread/new',
      name: 'NewForumThread',
      component: () => import('../views/forum/NewForumThreadView.vue'),
    },
    {
      path: '/wiki/article/:id',
      name: 'WikiArticle',
      component: () => import('../views/WikiView.vue'),
    },
    {
      path: '/conversation/new',
      name: 'NewConversation',
      component: () => import('../views/conversation/NewConversationView.vue'),
    },
    {
      path: '/conversation/:id',
      name: 'Conversation',
      component: () => import('../views/conversation/ConversationView.vue'),
    },
    {
      path: '/conversations',
      name: 'Conversations',
      component: () => import('../views/conversation/ConversationsView.vue'),
    },
  ],
})

export default router
