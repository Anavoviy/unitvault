import { A } from "@solidjs/router";
import { openUrl } from "@tauri-apps/plugin-opener";
import Container from "../ui/container.tsx";

export default function Hello() {
  
  return (
    <div class="flex flex-col min-w-screen w-screen min-h-screen h-screen bg-primary text-txtprimary">
      <Container class="flex flex-col">
        <header class="h-1/4 flex flex-col justify-center"> {/*Возможно и не стоит делать относительно... пойдет и жесткие отступы*/}
          <h1>💎 UnitVault</h1>
          <p class="mt-6 text-h3 text-center">
            Локальное моделирование юнит-экономики
          </p>
        </header>
        <main class="h-1/2 flex flex-col justify-between"> {/*Возможно и не стоит делать относительно... пойдет и жесткие отступы*/}
          <div class="mt-14 w-full flex justify-between">
            <A href="/open"
             class="w-[45%] bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary">
              📁 Открыть модель
              <p class="text-body text-center">Загрузить существующую модель</p>
            </A>
            <A href="/create"
             class="w-[45%] bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary">
              ✨ Создать новую модель
              <p class="text-body text-center">Начать с чистого листа</p>
            </A>
          </div>

          <div class="mt-10 mx-30">
            <h3 class="text-center">Недавние модели:</h3>
            <div class="rounded-[10px] bg-secondary mt-5 p-4 flex justify-center gap-5">
              <A href="/create"
                class="bg-tertiary rounded-lg py-6 px-6 min-w-40 duration-250 hover:bg-elevated">
                <h4 class="text-center text-h3 font-medium">Создать</h4>
                <img src="./src/assets/plus.png" class="w-7 h-7 mx-auto mt-2.5"/>
              </A>
            </div>
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