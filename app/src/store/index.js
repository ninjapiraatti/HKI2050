import { reactive } from 'vue'

const state = reactive({
  counter: 666,
  loggeduser: JSON.parse(localStorage.getItem('user')),
})

const methods = {
  increase() {
    state.counter++
  },
  decrease() {
    state.counter--
  },
  logout() {
    console.log("Logging out")
    return true
  },
  login(formData) {
    console.log("Logging in:" + formData)
    return true
  }
}

export default {
  state,
  methods
}