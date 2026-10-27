import {A, useNavigate} from "@solidjs/router";
import { openUrl } from "@tauri-apps/plugin-opener";
import {createSignal, onMount} from "solid-js";
import Container from "../ui/container.tsx";
import {invoke} from "@tauri-apps/api/core";

export default function Open() {
    const [pass, setPass] = createSignal("");
    const [filePath, setFilePath] = createSignal("");
    const navigate = useNavigate();
    const fileName = () => {
        let start = filePath().lastIndexOf("/") + 1;

        return filePath().substring(start);
    }

    const openModel = async () => {
        await invoke("load_file", {
            "file_path": filePath(),
            "password": pass()
        }).catch((e) => {
            if(e.toString() === "success"){
                //TODO: Pop-up с "успехом"
                navigate("/main");
            } else {
                //TODO: Сделать Pop-up ошибок
                console.log("Пароль не подошел");
            }

        }).then((res) => {
            //TODO: Сделать Pop-up ошибок
            console.log(res);
        });
    }
    

    onMount(() => {
       invoke("get_open_filepath").catch((e) => {
           //TODO: Сделать Pop-up ошибок
            console.log(e);
       }).then((path) => {
           setFilePath((path as string));
       })
    });



    return (
        <div class="min-w-screen min-h-screen h-screen w-screen bg-primary text-txtprimary">
            <Container class="flex flex-col">
                <header >
                    <A href="/" class="group self-start flex items-center gap-1.5">
                        <div class="w-6 h-6 relative">
                            <img src="./src/assets/left_arrow.png"
                                class="absolute w-6 h-6 duration-250 opacity-100 group-hover:opacity-0"/>
                            <img src="./src/assets/accent_left_arrow.png"
                                class="absolute w-6 h-6 duration-250 opacity-0 group-hover:opacity-100"/>
                        </div>
                        <p class="duration-250 group-hover:text-accent">Назад</p>
                    </A>
                </header>
                <main class="flex flex-col gap-10 h-full">
                    <div class="h-1/4 flex flex-col justify-center">
                        <h1>🔐 Защита</h1>
                        <h3 class="mt-6 text-center">Введите пароль</h3>
                    </div>

                    <div class="flex flex-col h-3/4">
                        <h2 class="text-h2 text-center font-semibold">Файл: <span class="font-normal">{fileName()}</span></h2>
                        <input type="password" class="mt-10 min-w-175 lg:w-4/5 self-center border px-10 py-4
                        rounded-[10px] placeholder:text-h2 placeholder:text-white
                        placeholder:font-semibold placeholder:text-center
                        text-center text-h2 font-semibold"
                               placeholder="•••" value={pass()}
                               onInput={(e) => setPass(e.target.value)}/>
                        <button class="self-center w-[45%] bg-secondary py-5 px-14 mt-18
                            rounded-[10px] text-center text-h3
                            duration-250 hover:bg-tertiary"
                            onClick={async () => {
                                await openModel();
                            }}>Открыть</button>
                    </div>
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
            </Container>
        </div>
    );
}