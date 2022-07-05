import { api } from '@root/api.js'
let store

function extractExistingTags(inputTags, allTags) {
  let filteredTags = allTags.filter( i => inputTags.includes( i.title ) )
  return filteredTags
}

function extractNewTags(inputTags, allTags) {
  let allTagTitles = allTags.map( i => i.title )
  console.log(allTagTitles)
  let filteredTags = inputTags.filter( i => !allTagTitles.includes( i ) )
  return filteredTags
}

export const utils = {
  saveTags: async function(tags, userId, contentId) {
    let existingTags = extractExistingTags(tags, store.state.tags)
    let newTags = extractNewTags(tags, store.state.tags)
    // console.log(newTags)
    // console.log(existingTags)    

    newTags.forEach( async tag => {
      const dataNewTag = {
        user_id: userId,
        title: tag,
      }
      let newTagRes = await api.tags.save(dataNewTag)
      const dataContentTag = {
        id: contentId,
        user_id: userId,
        tag_id: newTagRes.id,
      }
      let newContentTagRes = await api.content_tags.save(dataContentTag)
      if (!newContentTagRes.success) {
        console.log(newContentTagRes)
      }
    })

    existingTags.forEach( async tag => {
      const data = {
        id: contentId,
        user_id: userId,
        tag_id: tag.id,
      }
      await api.content_tags.save(data)
    })
  }
}

export default {
	install: (app, {storeApi}) => {
    store = storeApi
		app.provide('utils', utils)
	},
}