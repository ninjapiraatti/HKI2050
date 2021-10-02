import '../scss/main.scss'

import { createApp } from 'vue'
import api from '@root/api.js'
//import store from '@store'
import router from '@root/router.js'
import modal from '@root/plugins/vue-modal'
import FlashMessage from '@smartweb/vue-flash-message'
import App from '@root/App.vue'
import 'bootstrap'
import { Form, Field, ErrorMessage } from 'vee-validate'
import '@root/validation.js'

const app = createApp(App)
	//.use(store)
	.use(router)
	.use(api)
	.use(FlashMessage)
	.use(modal)
	.component('VForm', Form)
	.component('VField', Field)
	.component('ErrorMessage', ErrorMessage)

router.isReady()
	.then(() => app.mount('#hki2050'))
