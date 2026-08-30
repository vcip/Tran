import "./App.css"

import { invoke } from "@tauri-apps/api/core"
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { createSignal, onMount } from "solid-js"
import { render } from "solid-js/web"

const Settings = () => {
    const window = getCurrentWebviewWindow()
    const [host, setHost] = createSignal("https://translate.googleapis.com")
    const [proxy, setProxy] = createSignal("")
    const [savedHost, setSavedHost] = createSignal(
        "https://translate.googleapis.com"
    )
    const [savedProxy, setSavedProxy] = createSignal("")
    const [status, setStatus] = createSignal("")

    const reload = async () => {
        const currentHost = await invoke<string>("host")
        const currentProxy = await invoke<string>("proxy")
        setHost(currentHost || "https://translate.googleapis.com")
        setSavedHost(currentHost || "https://translate.googleapis.com")
        setProxy(currentProxy || "")
        setSavedProxy(currentProxy || "")
    }

    onMount(async () => {
        await reload()
        window.addEventListener("focus", async () => {
            await reload()
        })
    })

    const save = async () => {
        const nextHost = host().trim() || "https://translate.googleapis.com"
        const nextProxy = proxy().trim()

        await invoke("set_host", { host: nextHost })
        await invoke("set_proxy", { proxy: nextProxy })
        setSavedHost(nextHost)
        setSavedProxy(nextProxy)
        setStatus("Saved")
        setTimeout(async () => {
            await window.hide()
        }, 300)
    }

    const cancel = async () => {
        setHost(savedHost())
        setProxy(savedProxy())
        setStatus("Cancelled")
        setTimeout(async () => {
            await window.hide()
        }, 300)
    }

    const resetHost = () => {
        setHost("https://translate.googleapis.com")
    }

    const resetProxy = () => {
        setProxy("")
    }

    return (
        <div class="settings-window">
            <div class="settings-header">Tran Settings</div>

            <div class="settings-form">
                <label class="settings-row">
                    <span>Host</span>
                    <div class="settings-input-wrap">
                        <input
                            class="settings-input"
                            value={host()}
                            onInput={(e) =>
                                setHost(
                                    (e.currentTarget as HTMLInputElement).value
                                )
                            }
                            placeholder="https://translate.googleapis.com"
                        />
                        <button
                            class="settings-small-btn"
                            type="button"
                            onClick={resetHost}
                        >
                            Default
                        </button>
                    </div>
                </label>

                <label class="settings-row">
                    <span>Proxy</span>
                    <div class="settings-input-wrap">
                        <input
                            class="settings-input"
                            value={proxy()}
                            onInput={(e) =>
                                setProxy(
                                    (e.currentTarget as HTMLInputElement).value
                                )
                            }
                            placeholder="socks5://127.0.0.1:7890"
                        />
                        <button
                            class="settings-small-btn"
                            type="button"
                            onClick={resetProxy}
                        >
                            Clear
                        </button>
                    </div>
                </label>
            </div>

            <div class="settings-actions">
                <div class="settings-status">{status()}</div>
                <button class="settings-cancel" type="button" onClick={cancel}>
                    Cancel
                </button>
                <button class="settings-save" type="button" onClick={save}>
                    Save
                </button>
            </div>
        </div>
    )
}

render(() => <Settings />, document.getElementById("root") as HTMLElement)
