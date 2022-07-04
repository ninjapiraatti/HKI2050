import '../scss/main.scss'

import { createApp, toRef } from 'vue'
import store from '@store'
import api from '@root/api.js'
import router from '@root/router.js'
import colorScheme from '@root/plugins/vue-color-scheme'
import modal from '@root/plugins/vue-modal'
import utils from '@root/plugins/hkiUtils'
import FlashMessage from '@smartweb/vue-flash-message'
import App from '@root/App.vue'
import 'bootstrap'
import { Form, Field, ErrorMessage } from 'vee-validate'
import '@root/validation.js'

const app = createApp(App)
	//.use(store)
	.use(router)
	.use(api, {storeApi: store, routerApi: router})
	.use(FlashMessage)
	.use(modal)
	.use(utils, {storeApi: store})
	//app.provide('colorScheme', colorScheme)
	.use(colorScheme, { scheme: toRef(store.state, 'colorScheme') })
	//.use(colorScheme)
	.component('VForm', Form)
	.component('VField', Field)
	.component('ErrorMessage', ErrorMessage)

router.isReady()
	.then(() => app.mount('#hki2050'))

