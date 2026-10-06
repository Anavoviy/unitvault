import { A } from "@solidjs/router";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { createSignal, onMount, Show } from "solid-js"


export default function Open() {
    const [pass, setPass] = createSignal("");

    const [isError, setError] = createSignal(true);
    const [filePath, setFilePath] = createSignal("");

    onMount(() => {
        invoke("enter_filepath").catch((err) => setError(true)).then(e => setFilePath((e as string)));
    });

    return (
        <div class="flex flex-col min-w-screen min-h-screen bg-primary text-txtprimary p-10">
            <A href="/" class="group self-start flex items-center gap-1.5">
                <div class="w-6 h-6 relative">   
                    <img src="./src/assets/left_arrow.png" 
                        class="absolute w-6 h-6 duration-250 opacity-100 group-hover:opacity-0"/>
                    <img src="./src/assets/accent_left_arrow.png" 
                        class="absolute w-6 h-6 duration-250 opacity-0 group-hover:opacity-100"/>
                </div>
                <p class="duration-250 group-hover:text-accent">Назад</p>
            </A>
            <main class="flex flex-col">
                <h1>🔐 Защита</h1>
                <h3 class="mt-6 text-center">Введите пароль для открытия</h3>

                <input class="min-w-175 mt-28 self-center border px-10 py-4 rounded-[10px] placeholder:text-h2 placeholder:text-white placeholder:font-semibold placeholder:text-center text-center text-h2 font-semibold" placeholder="✻" value={pass()} onInput={(e) => setPass(e.target.value)}/>

                <Show when={isError()}>
                    <p class="text-xl font-medium text-center mt-4">Неверный пароль</p>
                    <p class="text-center mt-3">Забыли пароль? Данные нельзя восстановить.</p>
                    <button class="mt-5 self-center bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary w-127.5">Открыть модель</button>
                </Show>
                <Show when={!isError()}>
                    <button class="mt-25 self-center bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary w-127.5">Открыть модель</button>
                </Show>
            </main>
            <footer class="mt-auto w-full flex justify-center">
                <button class="font-medium hover:text-accent duration-250" 
                    onClick={
                    async () => 
                        await openUrl("https://github.com/Anavoviy")
                    }>
                    Developed by Anavoviy
                </button>
            </footer>
        </div>
    );
}