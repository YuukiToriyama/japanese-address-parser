import init, {Parser} from "../pkg/japanese_address_parser.js"

const inputTextArea = document.getElementById("input")

init().then(() => {
    document.getElementById("exec").addEventListener("click", () => {
        const input = inputTextArea.value
        alert("input: " + input)
        const parser = new Parser()
        parser.parse(input).then(result => {
            document.getElementById("result").appendChild(
                createRow(input, result)
            )
        })
    })
})

const createRow = (input, parseResult) => {
    const tr = document.createElement("tr")
    tr.appendChild(createCell(`<p>${input}</p>`))
    if (parseResult.error === undefined) {
        tr.appendChild(createCell("<p>成功</p>"))
    } else {
        tr.appendChild(createCell("<p>失敗</p>"))
    }
    tr.appendChild(createCell(`<p>${parseResult.address.prefecture}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.address.city}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.address.town}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.address.rest}</p>`))
    tr.appendChild(createCell(`<code>${JSON.stringify(parseResult, null, null)}</code>`))
    return tr
}

const createCell = (innerHtml) => {
    const td = document.createElement("td")
    td.innerHTML = innerHtml
    return td
}