import { createApp } from "vue"
import { createPinia } from "pinia"
import router from "./router"
import App from "../App.vue"
import "../style.css"
// Uzum UI os-theme: global styles import + providers in App.vue (UConfigProvider, UGlobalStyle, UMessageProvider, ULoadingBarProvider).
import "@uzum-tech/ui/es/styles"

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount("#app")
