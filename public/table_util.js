const createRow = (input, parseResult) => {
    const tr = document.createElement("tr")
    tr.appendChild(createCell(`<p>${input}</p>`))
    if (parseResult.error === undefined) {
        tr.appendChild(createCell("<p>✅</p>"))
    } else {
        tr.appendChild(createCell("<p>❌</p>"))
    }
    tr.appendChild(createCell(`<p>${parseResult.address.prefecture}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.address.city}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.address.town}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.address.rest}</p>`))
    tr.appendChild(createCell(`<code>${JSON.stringify(parseResult, null, 2)}</code>`, true))
    return tr
}

const createRowForNightlyPage = (input, parseResult) => {
    const tr = document.createElement("tr")
    tr.appendChild(createCell(`<p>${input}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.metadata.depth}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.prefecture}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.city}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.town}</p>`))
    tr.appendChild(createCell(`<p>${parseResult.rest}</p>`))
    tr.appendChild(createCell(`<code>${JSON.stringify(parseResult, null, 2)}</code>`, true))
    return tr
}

const createCell = (innerHtml, isCopyable = false) => {
    const td = document.createElement("td")
    td.innerHTML = innerHtml

    td.querySelectorAll("script").forEach(script => script.remove())
    td.querySelectorAll("*").forEach(el => {
        for (const attr of [...el.attributes]) {
            if (attr.name.startsWith("on")) {
                el.removeAttribute(attr.name)
            }
        }
    })

    if (isCopyable) {
        const target = td.querySelector("code")
        if (target) {
            target.style.cursor = "pointer"
            target.addEventListener("click", () => {
                navigator.clipboard.writeText(target.innerText).then(() => {
                    target.classList.add("copied")
                    setTimeout(() => {
                        target.classList.remove("copied")
                    }, 1000)
                })
            })
        }
    }
    return td
}