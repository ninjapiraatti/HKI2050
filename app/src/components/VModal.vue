<template>
	<div class="modal fade" ref="modal" :data-bs-backdrop="backdrop" :data-bs-keyboard='backdrop != "static"'>
		<div class="modal-dialog" :class='sizeClass'>
			<div class="modal-content shadow-lg">
				<div class="modal-header" v-if="title">
					<div class="modal-title h2">{{ title }}</div>
					<button v-if='backdrop != "static"' type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
				</div>
				<div ref="body" class="modal-body">
					<slot></slot>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import { Modal } from 'bootstrap'
//import { computed, onMounted } from '@vue/runtime-core'
import { ref, computed, onMounted } from "vue";
const sizeClasses = {
	sm: 'modal-sm',
	lg: 'modal-lg',
	xl: 'modal-xl',
}
export default {
	name: 'VModal',
	props: {
		title: String,
		backdrop: {
			type: [Boolean, String],
			default: true,
			validator: value => typeof value == 'boolean' || value == 'static',
		},
		size: {
			type: String,
			validator: value => Object.keys(sizeClasses).includes(value)
		},
		showAtStart: false,
	},
	setup(props, {emit}) {
		const sizeClass = computed(() => {
			return sizeClasses[props.size] || ''
		})
		const modal = ref(null)
		const body = ref(null)
		const showAtStart = true

		onMounted(() => {
			let modalElement = modal.value
			let modalBodyElement = body.value 
			console.log(modalElement)
			let modalThing = Modal.getOrCreateInstance(modalElement)
			console.log(modalThing)
			console.log(body)
			modalElement.addEventListener('hide.bs.modal', () => { emit('modal-hiding') })
			modalElement.addEventListener('hidden.bs.modal', () => { emit('modal-hidden') })
			modalElement.addEventListener('show.bs.modal', () => { emit('modal-showing') })
			modalElement.addEventListener('shown.bs.modal', () => {
				emit('modal-shown')
				const selectors = [
					'input, select',
					'button.btn-primary',
					'button',
				]
				for (const selector of selectors) {
					const input = modalBodyElement.querySelector(selector)
					if (input) {
						input.focus()
						break
					}
				}
			})

			if (showAtStart) modalThing.show()
		})

		return {
			modal,
			body,
			sizeClass,
		}
	}
}
</script>
