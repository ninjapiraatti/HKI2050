let store, router, flashMessage

const errorMessages = {
	UniqueViolation: 'Item already exists',
}

const debounceFlashMessage = {}

function splitPascalCase(word) {
	const pattern = /($[a-z])|[A-Z][^A-Z]+/g
	const words = word.match(pattern).join(" ")
	return words[0] + words.slice(1).toLowerCase()
}

function debounce(func, timeout = 300) {
	let timer
	return (...args) => {
		if (!timer) func.apply(this, args)
		clearTimeout(timer)
		timer = setTimeout(() => {
			timer = undefined
		}, timeout)
	}
}

async function handleHttpStatus(response) {
	if (response.ok) return response

	let errorMessage = response.statusText

	let error = await response.json().catch(() => {})
	if (error) {
		error = error.error_type
		switch (error) {
			default:
				errorMessage = errorMessages[error] || splitPascalCase(error)
				console.error(error)
		}
	}

	if (!(errorMessage in debounceFlashMessage)) {
		debounceFlashMessage[errorMessage] = debounce(() => flashMessage.show({
			type: 'error',
			title: `Error ${response.status}`,
			text: errorMessage,
			time: 1000,
		}))
	}

	debounceFlashMessage[errorMessage]()

	switch (response.status) {
		case 401:
			store.dispatch('setUser', null)
			if (router.currentRoute.value.name !== 'login') {
				router.push({ name: 'login', query: {
					redirect: router.currentRoute.value.fullPath,
				}})
			}
			break
		case 500:
			router.push({ name: 'error', params: {
				title: 'Error 500',
				message: errorMessage,
			}})
			break
	}

	throw Error(response.statusText)
}

const populateUrl = (url, data) => {
	const isObject = typeof data === 'object' && data !== null && !Array.isArray(data)

	return url.replace(/\{[^}]*?\}/g, tag => {
		const mandatory = !tag.includes('?')
		tag = tag.slice(1, mandatory ? -1 : -2)

		const value = isObject
			? data[tag]
			: data

		if (mandatory && !value) {
			throw Error(`No "${tag}" provided for url "${url}"`)
		}

		return value || ''
	})
}

const prepareBody = body => {
	for (const key in body) {
		// Back end want's YYYY-MM-DD dates
		if (body[key] instanceof Date) body[key] = [
			body[key].getFullYear(),
			body[key].getMonth() + 1,
			body[key].getDate(),
		].map(nr => String(nr).padStart(2, 0)).join('-')
	}

	return body
}

const request = ({ url, onError, ...options } = {}) => fetch(url.replace(/\/+$/, ''), options)
	.then(handleHttpStatus)
	.catch(error => {
		if (onError !== undefined) return onError
		throw error
	})

const sendJson = ({ method = 'POST', ...args } = {}) => request({
	...args,
	method,
	headers: { 'Content-Type': 'application/json' },
	body: JSON.stringify(prepareBody(args.body)),
})

const returnBoolean = promise => promise.then(() => true).catch(() => false)
const returnObject = promise => promise.then(response => response.json()).catch(() => null)
const returnArray = promise => promise.then(response => response.json()).catch(() => [])

const create = url => body => returnObject(sendJson({ url: populateUrl(url, body), method: 'POST', body }))
const update = url => body => returnObject(sendJson({ url: populateUrl(url, body), method: 'PUT', body }))

const save = urls => {
	if (typeof urls == 'string') urls = { create: urls.replace('/{id}', ''), update: urls }

	// Prioritize update because it could have the same key that is used to create
	// create /api/user/{user_id}/thing: { user_id: 'abc123', name: 'Foo' }
	// update /api/user/thing/{id}:      { user_id: 'abc123', name: 'Bar', id: '321cba' }
	const updateKey = 'update' in urls
		? (urls.update.match(/{([^}]*?)}/) || []).pop()
		: null

	const c = create(urls.create)
	const u = update(urls.update)

	return data => data[updateKey] ? u(data) : c(data)
}

const getArray = url => data => returnArray(request({ url: populateUrl(url, data) }))
const getObject = url => data => returnObject(request({ url: populateUrl(url, data) }))

const remove = url => data => returnBoolean(
	typeof data == 'string'
		? request({ url: populateUrl(url, data), method: 'DELETE' })
		: sendJson({ url: populateUrl(url, data), method: 'DELETE', body: data })
)

export const api = {
	users: {
		get: async (data = {}) => {
			if (!data.id) return getArray('/api/users?is_include_skills=true')()
			return getObject('/api/users/{id}')(data)
		},

		save: save('/api/users/{id}'),
		delete: remove('/api/users/{id}'),

		files: {
			get: (data = {}) => {
				if ('user_id' in data) return getArray('/api/users/{user_id}/uploads')(data)
				return populateUrl('/api/users/uploads/{id}', data)
			},

			save: data => {
				const body = new FormData()
				body.append('user_id', data.user_id)
				if (data.files.length) data.files.forEach(file => body.append('files[]', file))
				return returnBoolean(request({
					url: populateUrl('/api/users/{user_id}/uploads', data),
					method: 'POST',
					body,
				}))
			},

			delete: remove('/api/users/uploads/{id}'),
		},
		characters: {
			get: async (data = {}) => {
				const characters = await getArray('/api/users/{user_id}/characters')(data)
	
				return characters
			},
	
			save: save({
				create: '/api/users/{user_id}/characters',
				update: '/api/users/characters/{id}',
			}),

			delete: remove('/api/users/characters/{id}'),
		},

		password: {
			requestReset: body => returnBoolean(sendJson({ url: '/api/resetpassword', body })),
			save: body => returnBoolean(sendJson({ url: '/api/updatepassword', method: 'PUT', body })),
		},

		registration: {
			invite: body => sendJson({ url: '/api/invitations', body }),
			confirm: body => returnBoolean(sendJson({ url: populateUrl('/api/register/{id}', body), body })),
		},

		log: {
			in: async body => {
				await sendJson({ url: '/api/auth', body })
				return returnObject(request({ url: '/api/auth' }))
			},

			out: () => request({ url: '/api/auth', method: 'DELETE' }),
		},
	},
	characters: {
		get: async (data = {}) => {
			const characters = await getArray('/api/users/{id}/characters')(data)

			characters.forEach(reservation => {
				if (reservation.begin_time) reservation.begin_time = new Date(reservation.begin_time)
				if (reservation.end_time) reservation.end_time = new Date(reservation.end_time)
			})

			return characters
		},

		save: save('/api/characters/{id}'),
		delete: remove('/api/characters/{id}'),
	},
}

export default {
	install: (app, options) => {
		//store = app.config.globalProperties.$store
		//router = app.config.globalProperties.$router
		//flashMessage = app.config.globalProperties.$flashMessage

		//app.config.globalProperties.$api = api
		// All of these are now imported per component. Maybe refactor the way modal is done?

		app.provide('api', api)
		//app.provide('confirm', confirm(modal))
	},
}
