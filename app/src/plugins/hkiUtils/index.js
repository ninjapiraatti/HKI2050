let store

function extractExistingTags(inputTags, allTags) {
  let filteredTags = allTags.filter( i => inputTags.includes( i.title ) )
  console.log(filteredTags);
}

export const utils = {
  saveTags: function(tags) {
    console.log(store)
    let existingTags = extractExistingTags(tags, store.state.tags)
    let newTags = tags.filter( i => !existingTags.includes( i ) )
    console.log(newTags);
  }
}

export default {
	install: (app, {storeApi }) => {
    store = storeApi
		app.provide('utils', utils)
	},
}