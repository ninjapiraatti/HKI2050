let store

function extractExistingTags(inputTags, allTags) {
  let filteredTags = allTags.filter( i => inputTags.includes( i.title ) )
  return filteredTags.map( i => i.title )
}

export const utils = {
  saveTags: function(tags) {
    let existingTags = extractExistingTags(tags, store.state.tags)
    let newTags = tags.filter( i => !existingTags.includes( i ) )
    console.log(newTags)
    console.log(existingTags)
  }
}

export default {
	install: (app, {storeApi }) => {
    store = storeApi
		app.provide('utils', utils)
	},
}