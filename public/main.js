import init, {Parser} from "../pkg/japanese_address_parser.js"

const inputTextArea = document.getElementById("input")
const outputTextArea = document.getElementById("output")

init().then(() => {
    document.getElementById("exec").addEventListener("click", () => {
        const input = inputTextArea.value
        alert("input: " + input)
        const parser = new Parser()
        parser.parse(input).then(result => {
            outputTextArea.value = JSON.stringify(result, null, "\t")
        })
    })
})