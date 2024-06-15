const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

const progressDiv = document.getElementById('pages')
const msgEl = document.getElementById('greet-msg')

// Send file to Rust Handler
async function sendToRust() {
  try {
    await invoke('handle_pdf')
      .then(() => {
        alert('Busca concluida')
      })
  } catch (error) {
    console.error(error);
  }
}

document.getElementById('btnSearch').addEventListener('click', sendToRust)
document.getElementById('clearSearch').addEventListener('click', () => {
  msgEl.innerText = ''
})

listen('progress', (event) => {
  progressDiv.innerText = event.payload
})

listen('match_found', (event) => {
  msgEl.innerHTML += `<div>${event.payload}</div>`
})
