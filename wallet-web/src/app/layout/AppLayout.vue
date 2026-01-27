<script setup lang="ts">
import { RouterView } from "vue-router"
import { watch } from "vue"
import { ULayout, ULayoutContent, ULayoutHeader, ULayoutSider, USpin, UAlert, useLoadingBar, useMessage } from "@uzum-tech/ui"
import SidebarNav from "./SidebarNav.vue"
import TopBar from "./TopBar.vue"
import PageContainer from "./PageContainer.vue"
import { useSessionStore } from "../stores/session"
import { registerMessageApi } from "../../shared/ui/notifications"

const session = useSessionStore()
const loadingBar = useLoadingBar()
const message = useMessage()

registerMessageApi(message)

watch(
  () => session.isLoading,
  (isLoading) => {
    if (!loadingBar) return
    if (isLoading) {
      loadingBar.start()
    } else {
      loadingBar.finish()
    }
  },
)
</script>

<template>
  <ULayout class="app-shell">
    <ULayoutSider bordered class="app-sider" width="240">
      <SidebarNav />
    </ULayoutSider>
    <ULayout>
      <ULayoutHeader class="app-header">
        <TopBar />
      </ULayoutHeader>
      <ULayoutContent class="app-content">
        <PageContainer>
          <UAlert
            v-if="session.alert"
            :type="session.alert.type"
            closable
            class="app-alert"
            @close="session.clearAlert"
          >
            {{ session.alert.message }}
          </UAlert>
          <USpin :show="session.isLoading">
            <RouterView />
          </USpin>
        </PageContainer>
      </ULayoutContent>
    </ULayout>
  </ULayout>
</template>

<style scoped>
.app-shell {
  min-height: 100vh;
}

.app-sider {
  padding: 16px 12px;
}

.app-header {
  padding: 12px 24px;
}

.app-content {
  padding: 24px;
}

.app-alert {
  margin-bottom: 16px;
}
</style>
