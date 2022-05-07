import { Store, Backend } from "story"

const store = Store.init(Backend.Text)
console.log(store.get_backend())
