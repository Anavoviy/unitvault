import { A, Navigate } from "@solidjs/router";
import { openUrl } from "@tauri-apps/plugin-opener";

export default function Hello() {
  
  return (
    <div class="flex flex-col min-w-screen min-h-screen bg-primary text-txtprimary p-10">
      <div class="max-w-290 min-w-200 w-200 mx-auto">
        <h1>💎 UnitVault</h1>
        <p class="mt-6 text-h3 text-center">
          Локальное моделирование юнит-экономики
        </p>
        
        <div class="mt-14 w-full flex justify-between">
          <A href="/open"
           class="bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary">
            📁 Открыть модель
            <p class="text-body text-center">Загрузить существующую модель</p>
          </A>
          <A href="/create"
           class="bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary">
            ✨ Создать новую модель
            <p class="text-body text-center">Начать с чистого листа</p>
          </A>
        </div>

        <div class="mt-10 w-200 mx-auto">
          <h3 class="text-center">Недавние модели:</h3>
          <div class="rounded-[10px] bg-secondary mt-5 p-4 flex justify-center gap-5">
            <A href="/create"
              class="bg-tertiary rounded-lg py-6 px-6 min-w-40 duration-250 hover:bg-elevated">
              <h4 class="text-center text-h3 font-medium">Создать</h4>
              <img src="./src/assets/plus.png" class="w-7 h-7 mx-auto mt-2.5"/>
            </A>
          </div>
        </div>
      </div>
      
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