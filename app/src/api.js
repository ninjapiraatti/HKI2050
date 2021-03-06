import { flashMessage } from '@smartweb/vue-flash-message';
let store, router


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
	//console.log(store)
	//console.log(router)
	//console.log(response)
	if (response.ok) return response

	let errorMessage = response.statusText

	let error = await response.json().catch(() => {})
	if (error) {
		error = error.error_type
		switch (error) {
			default:
				errorMessage = errorMessages[error] || splitPascalCase(error)
				//console.error(error)
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
			store.methods.setUser(null)
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
		// Back end wants YYYY-MM-DD dates
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

		characters: {
			get: async (data = {}) => {
				return await getArray('/api/users/{user_id}/characters')(data)
			},
	
			save: save({
				create: '/api/users/{user_id}/characters',
				update: '/api/users/characters/{id}',
			}),

			delete: remove('/api/users/characters/{id}'),
		},

		articles: {
			get: async (data = {}) => {
				return await getArray('/api/users/{user_id}/articles')(data)
			},
	
			save: save({
				create: '/api/users/{user_id}/articles',
				update: '/api/users/articles/{id}',
			}),

			delete: remove('/api/users/articles/{id}'),
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
			const characters = await getArray('/api/characters')(data)
			return characters
		},
	},

	articles: {
		get: async (data = {}) => {
			//console.log(data)
			if (!data.id) return getArray('/api/articles')()
			return getObject('/api/articles/{id}')(data)
		},
	},

	tags: {
		get: async (data = {}) => {
			return await getArray('/api/tags')(data)
		},

		save: save({
			create: '/api/tags',
			update: '/api/tags/{id}',
		}),

		delete: remove('/api/tags/{id}'),
	},

	content_tags: {
		get: async (data = {}) => {
			return await getArray('/api/content-tags/{id}')(data) // {id} here is content id, not tag id
		},

		save: save({
			create: '/api/content-tags/{id}', // {id} here is content id, not tag id
		}),

		delete: remove('/api/content-tags/{id}'),
	},
}

export default {
	install: (app, { storeApi, routerApi, flashMessageApi }) => {
		store = storeApi
		router = routerApi
		//console.log(store)
		app.provide('api', api)
	},
}
