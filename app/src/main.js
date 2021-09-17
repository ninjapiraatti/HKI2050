import '../scss/main.scss'

import { createApp } from 'vue'
import router from './router.js'
import App from './App.vue'
import 'bootstrap'

const app = createApp(App)
	.use(router)

router.isReady()
	.then(() => app.mount('#hki2050'))
