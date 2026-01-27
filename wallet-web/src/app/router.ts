import { createRouter, createWebHistory, type RouteLocationNormalized, type RouteRecordRaw } from "vue-router"
import DashboardPage from "../pages/DashboardPage.vue"
import AdminPage from "../pages/AdminPage.vue"
import WalletPage from "../pages/WalletPage.vue"
import ReceiptPage from "../pages/ReceiptPage.vue"

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "dashboard",
    component: DashboardPage,
    meta: { title: "Dashboard" },
  },
  {
    path: "/admin",
    name: "admin",
    component: AdminPage,
    meta: { title: "Admin" },
  },
  {
    path: "/wallet",
    name: "wallet",
    component: WalletPage,
    meta: { title: "Wallet" },
  },
  {
    path: "/receipt",
    name: "receipt",
    component: ReceiptPage,
    meta: { title: "Receipt" },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.afterEach((to: RouteLocationNormalized) => {
  const title = to.meta?.title ? `Uzum Wallet • ${to.meta.title}` : "Uzum Wallet"
  if (typeof document !== "undefined") {
    document.title = title
  }
})

export default router
