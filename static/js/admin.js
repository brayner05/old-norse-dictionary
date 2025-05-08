
// Configuration for the admin page
const admin_config = {
    api_url: "/api",
}

const admin_buttons = document.querySelectorAll("main.admin-panel ul button")

const admin_panel = {
    active_panel: admin_buttons[0]
}

for (const button of admin_buttons) {
    button.addEventListener("click", e => {
        admin_panel.active_panel.classList.remove("active")
        button.classList.add("active")
        admin_panel.active_panel = button
    })
}

const get_all_words = async () => {
    const response = await fetch(`${admin_config.api_url}/words`)
    const data = await response.json()
    return data
}