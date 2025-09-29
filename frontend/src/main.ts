import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import PrimeVue from 'primevue/config'
import Aura from '@primeuix/themes/aura'
import ToastService from 'primevue/toastservice'
import Tooltip from 'primevue/tooltip'

const app = createApp(App)

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.darkmode',
    },
  },
})
app.use(router)
app.use(ToastService)
app.directive('tooltip', Tooltip)

app.mount('#app')

export function showToast(
  title: string,
  detail: string,
  severity: string,
  life?: number,
  closable: boolean = true,
  // group: string = 'tr',
): void {
  app.config.globalProperties.$toast.add({
    severity: severity,
    summary: title,
    detail: detail,
    life: life,
    closable: closable,
    group: 'tr',
  })
}
