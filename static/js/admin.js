
// Configuration for the admin page
const admin_config = {
    api_url: "/api",
}

class Word {
    constructor(native_word, english_translation) {
        this.native_word = native_word
        this.english_translation = english_translation
    }

    toString() {
        return `${this.native_word} (${this.english_translation})`
    }
}

const admin_buttons = document.querySelectorAll("main.admin-panel ul button")

const admin_panel = {
    active_button: admin_buttons[0],
    content_panel: document.getElementById("content")
}

for (const button of admin_buttons) {
    button.addEventListener("click", async e => {
        if (e.target == admin_panel.active_button) {
            return
        }

        admin_panel.active_button.classList.remove("active")
        button.classList.add("active")
        admin_panel.active_button = button

        await switch_panel()
    })
}

const get_all_words = async () => {
    const response = await fetch(`${admin_config.api_url}/words`)
    const data = await response.json()
    return data.map(datum => new Word(datum.native_word, datum.english_translation))
}

const switch_panel = async () => {
    let words = []

    switch (admin_panel.active_button.id) {
        case "admin-button-view-entries": {
            words = await get_all_words()
        }
    }

    const list = document.createElement("div")
    list.classList.add("content-list")

    for (const word of words) {
        const item = document.createElement("div")
        item.classList.add("content-list-item")
        item.innerText = word.toString()
        list.appendChild(item)
    }

    admin_panel.content_panel.appendChild(list)
}