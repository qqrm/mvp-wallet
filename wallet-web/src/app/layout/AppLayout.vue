<script setup lang="ts">
import { RouterView } from "vue-router"
import { watch } from "vue"
import { USpin, UAlert, useLoadingBar, useMessage } from "@uzum-tech/ui"
import SidebarNav from "./SidebarNav.vue"
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
    if (isLoading) loadingBar.start()
    else loadingBar.finish()
  },
)
</script>

<template>
  <div class="app-shell">
    <aside class="app-sider">
      <SidebarNav />
    </aside>

    <main class="app-main">
      <div class="app-content">
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
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-shell {
  min-height: 100vh;
  display: flex;
  background: var(--bg);
}

.app-sider {
  width: 256px;
  padding: 18px 14px;
  background: var(--bg-elev);
  border-right: 1px solid var(--border);
}

.app-main {
  flex: 1;
  min-width: 0;
}

.app-content {
  padding: 18px;
}

.app-alert {
  margin-bottom: 16px;
}

@media (max-width: 920px) {
  .app-sider {
    width: 220px;
  }
}

@media (max-width: 720px) {
  .app-shell {
    flex-direction: column;
  }

  .app-sider {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--border);
  }

  .app-content {
    padding: 12px;
  }
}
</style>
