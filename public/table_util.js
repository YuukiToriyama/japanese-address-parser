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
    td.addEventListener("click", () => {
        navigator.clipboard.writeText(td.innerText).then(() => {
            console.log(td.innerText)
        })
    })
    return td
}