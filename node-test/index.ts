import { Store, Backend } from "story"

const store = Store.new(Backend.Simple, {})

store.put("foo", "1")
store.put("bar", "2")
console.log(store.get("foo"))
store.delete("foo")
console.log(store.get("foo"))
console.log(store.get("bar"))
