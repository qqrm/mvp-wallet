<script setup lang="ts">
import { ref } from "vue"
import AdminConsole from "./components/AdminConsole.vue"
import UserConsole from "./components/UserConsole.vue"

const tab = ref<"admin" | "user">("admin")
</script>

<template>
  <div class="page">
    <header class="header">
      <div class="brand">
        <div class="logo-dot"></div>
        <div class="title">Uzum Wallet MVP</div>
      </div>

      <div class="tabs">
        <button class="tab" :class="{ active: tab === 'admin' }" @click="tab = 'admin'">Admin</button>
        <button class="tab" :class="{ active: tab === 'user' }" @click="tab = 'user'">User</button>
      </div>
    </header>

    <main class="grid">
      <AdminConsole v-if="tab === 'admin'" class="span2" />
      <UserConsole v-else class="span2" />
    </main>
  </div>
</template>

<style>
:root {
  --bg: #0b0f1a;
  --card: #0f1628;
  --text: #e6ecff;
  --muted: #9aa6c7;
  --border: rgba(255, 255, 255, 0.08);
  --accent: #6aa4ff;
  --accent2: #71ffd8;
  --danger: #ff6a6a;
}

* { box-sizing: border-box; }
html, body { height: 100%; }
body {
  margin: 0;
  background: radial-gradient(1200px 800px at 15% 10%, rgba(106, 164, 255, 0.18), transparent 60%),
              radial-gradient(1200px 800px at 80% 20%, rgba(113, 255, 216, 0.10), transparent 55%),
              var(--bg);
  color: var(--text);
  font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, "Apple Color Emoji", "Segoe UI Emoji";
}

.page { max-width: 1200px; margin: 0 auto; padding: 16px 16px 40px; }

.header {
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  border: 1px solid var(--border);
  background: rgba(15, 22, 40, 0.7);
  border-radius: 16px;
  backdrop-filter: blur(12px);
}

.brand { display: flex; gap: 10px; align-items: center; }
.logo-dot {
  width: 12px; height: 12px; border-radius: 999px;
  background: linear-gradient(135deg, var(--accent), var(--accent2));
  box-shadow: 0 0 0 4px rgba(106, 164, 255, 0.12);
}
.title { font-weight: 700; letter-spacing: 0.2px; }

.tabs { display: flex; gap: 8px; }
.tab {
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text);
  padding: 8px 12px;
  border-radius: 12px;
  cursor: pointer;
}
.tab.active {
  border-color: rgba(106, 164, 255, 0.5);
  background: rgba(106, 164, 255, 0.10);
}

.grid {
  margin-top: 16px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 14px;
}
@media (max-width: 980px) {
  .grid { grid-template-columns: 1fr; }
  .span2 { grid-column: auto; }
}

.card {
  border: 1px solid var(--border);
  background: rgba(15, 22, 40, 0.75);
  border-radius: 16px;
  padding: 14px;
  box-shadow: 0 10px 30px rgba(0,0,0,0.25);
}

.span2 { grid-column: 1 / -1; }

.card-title { font-weight: 700; margin-bottom: 12px; }
.muted { color: var(--muted); }
.label { color: var(--muted); min-width: 88px; }

.row { display: flex; gap: 10px; align-items: center; margin: 8px 0; flex-wrap: wrap; }
.input {
  background: rgba(255,255,255,0.03);
  color: var(--text);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 10px 12px;
  outline: none;
  min-width: 160px;
}
.input:focus { border-color: rgba(106, 164, 255, 0.55); }

.btn, .btn-primary {
  border-radius: 12px;
  padding: 10px 12px;
  cursor: pointer;
  border: 1px solid var(--border);
  background: rgba(255,255,255,0.03);
  color: var(--text);
}
.btn-primary {
  border-color: rgba(106, 164, 255, 0.55);
  background: rgba(106, 164, 255, 0.16);
}
.btn:disabled, .btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }

.divider { height: 1px; background: var(--border); margin: 12px 0; }

.error {
  margin-top: 10px;
  border: 1px solid rgba(255, 106, 106, 0.35);
  background: rgba(255, 106, 106, 0.12);
  padding: 10px 12px;
  border-radius: 12px;
  color: #ffd2d2;
}

.balances { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }
@media (max-width: 980px) { .balances { grid-template-columns: 1fr; } }
.balance {
  border: 1px solid var(--border);
  background: rgba(255,255,255,0.03);
  padding: 12px;
  border-radius: 14px;
}
.bal-cur { font-weight: 700; }
.bal-num { font-size: 20px; margin-top: 6px; }
.bal-sub { color: var(--muted); margin-top: 2px; font-size: 13px; }

.total { margin-top: 10px; display: flex; justify-content: space-between; align-items: baseline; }
.total-num { font-size: 20px; font-weight: 700; }

.txs { display: flex; flex-direction: column; gap: 10px; margin-top: 10px; }
.tx {
  border: 1px solid var(--border);
  background: rgba(255,255,255,0.03);
  border-radius: 14px;
  padding: 12px;
  display: flex;
  gap: 12px;
  justify-content: space-between;
}
.tx-desc { font-weight: 600; }
.tx-meta { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 4px; align-items: center; }
.pill {
  font-size: 12px;
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 3px 8px;
  color: var(--muted);
}
.tx-right { text-align: right; min-width: 200px; }
.tx-amt { font-weight: 700; font-size: 18px; }
.tx-amt.neg { color: #ff9090; }
.tx-id { font-size: 12px; margin-top: 2px; }
.hint { margin-top: 8px; font-size: 13px; }
</style>
