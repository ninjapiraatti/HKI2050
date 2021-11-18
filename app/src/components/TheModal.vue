<template>
	<div v-if='modals.length'>
		<VModal
			:ref='id(modal)'
			v-for='modal in modals'
			:key='modal.id'
			:showAtStart='true'
			:title='modal.title'
			:backdrop='modal.backdrop'
			:size='modal.size'
			@modal-hidden='removeModal(modal)'>
			<component :is='modal.component' v-bind='modal.props' @success='onSuccess(modal, $event)' />
		</VModal>
	</div>
</template>

<script>
import VModal from '@components/VModal.vue'
export default {
	name: 'TheModal',
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
			this.$refs[this.id(modal)].hide()
		}

		return {
			id,
			removeModal,
			onSuccess,
		}
	},
	components: {
		VModal,
	},
}
</script>
