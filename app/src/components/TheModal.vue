<template>
	<div v-if='modals.length' :class="sayFoo">
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
	components: {
		VModal,
	},
	// Even though onSuccess is called in template it doesn't need to be here because we're using plugin vue-modal.
}
</script>
