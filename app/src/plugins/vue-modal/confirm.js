import FormConfirm from '@forms/FormConfirm.vue'
import { api } from '@root/api.js'

export default modal => {
	const confirm = ({ title, ok = 'OK', cancel = 'Cancel' } = {}) => modal({
		component: FormConfirm,
		title,
		props: {
			ok,
			cancel,
		},
	})

	confirm.delete = async (type, data) => {
		let title, apiCall
	
		switch (type) {	
			case 'user':
				title = 'profile'
				apiCall = api.users.delete.bind(null, data.id)
				break
		}
	
		const confirmed = await confirm({
			title: `Delete ${title}?`,
			ok: 'Delete',
		})
	
		return confirmed
			? apiCall()
			: confirmed
	}

	return confirm
}
