import { ref, shallowRef } from 'vue'
import TheModal from '@components/TheModal.vue'
import confirm from './confirm.js'

export default {
	install: (app, options) => {
		let id = 0
		const modals = ref([])

		app.component('TheModal', {
			...TheModal,
			setup() {
				function id(modal) {
					return `modal-${modal.id}`
				}
		
				function removeModal(modal) {
					this.modals.splice(this.modals.indexOf(modal), 1)
					modal.resolve(null)
				}
		
				function onSuccess(modal, payload) {
					modal.resolve(payload)
					console.log("Great success")
				}

				function sayFoo() {
					return "FOOBAR"
				}
		
				return {
					id,
					removeModal,
					onSuccess,
					modals,
					sayFoo
				}
			}
		})

		const modal = ({ title, component, props = {}, backdrop = true, size } = {}) => {
			console.log(title)
			if ('props' in component) props = Object.keys(component.props).reduce((used, key) => ({ ...used, [key]: props[key] }), {})

			return new Promise(resolve => {
				modals.value.push({
					id: ++id,
					resolve,
					title,
					component: shallowRef(component),
					props,
					backdrop,
					size,
				})
			})
		}

		app.provide('modal', modal)
		//app.config.globalProperties.$modal = modal
		//app.config.globalProperties.$confirm = confirm(modal)
	},
}
