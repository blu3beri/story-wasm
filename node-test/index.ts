import { Store, Backend, BackendOptions } from "story"

const store = Store.new(Backend.Text, {})
console.log(store.get("foop"))
