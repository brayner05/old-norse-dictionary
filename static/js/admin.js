
// Configuration for the admin page
const admin_config = {
    api_url: "/api",
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
    const response = await fetch(`${admin_config.api_url}/word`)
    const data = await response.json()
    return data
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
        item.innerText = word
        list.appendChild(item)
    }

    admin_panel.content_panel.appendChild(list)
}