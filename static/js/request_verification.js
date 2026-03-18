$(document).foundation()
import * as utils from './utils.js'
import * as globals from './globals.js'


/**
 * Functions for the request verification page
 **/


let msgs = []

// SHOW/HIDE ERROR BOX
const hide_msg_box = () =>
    document.getElementById("msg_box").classList.add('hidden')

const show_msg_box = () => {
    const msg_box = document.getElementById("msg_box")
    msg_box.innerHTML = "";

    for (let msg of msgs) {
        const msg_p = "<p>" + msg + "</p>"
        msg_box.innerHTML += msg_p
    }

    msg_box.classList.remove('hidden')
    msgs = []
}


async function request_new_code() {
    console.log("REQUESTING NEW CODE")
    const route = "/req_new_code"
    let email = document.getElementById("email_req").value
    let inputs = {
        "email": email.toString()
    }

    await utils.fetch_json_post(route, inputs)
    .then(response => {
        if (!response.ok) {
            response.json().then(data => {
                const msg = !!data.error && !!data.code ?
                    "<h3>" + data.code + "</h3><p>" + data.error + "</p>" :
                    "Error Occurred"
                msgs.push(msg)
                show_msg_box()
            })

            throw new Error("Inputs invalid or server error.")
        }
        return response.json()
    }).then(update_data => {
        if (!!update_data.message) {
            msgs.push(update_data.message)
            show_msg_box()
        }        
    }).catch(error => {
        console.log('Error: ', error)
    })
}



// Add event listeners

document.addEventListener('DOMContentLoaded', () => hide_msg_box())
document.getElementById('email').addEventListener(
    'keydown', (e) => (e.key === 'Enter') && request_new_code())
document.getElementById('submit_btn').addEventListener(
    'click', () => request_new_code())

