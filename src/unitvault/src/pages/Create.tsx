import { A } from "@solidjs/router";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { createSignal } from "solid-js";

function numberWithSpaces(x: number): string {
  return x.toFixed(0).replace(/\B(?=(\d{3})+(?!\d))/g, " ");
}

export default function Create() {
    const [name, setName] = createSignal("");
    const [type, setType] = createSignal(0);

    const [cac, setCac] = createSignal(5000);
    const [arpu, setArpu] = createSignal(1000);
    const [margin, setMargin] = createSignal(15);
    const [churnrate, setChurnrate] = createSignal(4);
    

    let selectType = (event: Event, num: number) => {
        event.preventDefault();
        setType(num);
    }

    //Select CAC
    const [cacProgress, setCacProgress] = createSignal(22.36);
    let progressCac: HTMLDivElement | undefined;
    let inputRangeCac: HTMLInputElement | undefined;
    let changeCac = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressCac) {
            progressCac.style.width = `${value}%`;
            setCacProgress(value);
        }
        
        setCac(Math.trunc(value * 10 * value));
        
    }
    let inputCac = (event: InputEvent) => {
        let cac = Number(event.currentTarget.value);

        if(cac < 0 || cac > 100000) {

        }
        else {
            let value = Math.sqrt(cac / 10);

            if (progressCac && inputRangeCac) {
                progressCac.style.width = `${value}%`;
                setCacProgress(value);
                inputRangeCac.value = `${value}`;
            }

            setCac(cac);
        }
    }

    // Select ARPU
    const [arpuProgress, setArpuProgress] = createSignal(1);
    let progressArpu: HTMLDivElement | undefined;
    let inputRangeArpu: HTMLInputElement | undefined;
    let changeArpu = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressArpu) {
            progressArpu.style.width = `${value}%`;
            setArpuProgress(value);
        }
        
        setArpu(Math.trunc(value * 100 * value));
        
    }
    let inputArpu = (event: InputEvent) => {
        let arpu = Number(event.currentTarget.value);

        if(arpu < 0 || arpu > 1000000) {

        }
        else {
            let value = Math.sqrt(arpu / 100);

            if (progressArpu && inputRangeArpu) {
                progressArpu.style.width = `${value}%`;
                setArpuProgress(value);
                inputRangeArpu.value = `${value}`;
            }

            setArpu(arpu);
        }
    }

    // Select Margin
    let progressMargin: HTMLDivElement | undefined;
    let inputRangeMargin: HTMLInputElement | undefined;
    let changeMargin = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressMargin) {
            progressMargin.style.width = `${value}%`;
        }
        
        setMargin(value);
        
    }
    let inputMargin = (event: InputEvent) => {
        let margin = Number(event.currentTarget.value);

        if(margin < 0 || margin > 100) {

        }
        else {
            if (progressMargin && inputRangeMargin) {
                progressMargin.style.width = `${margin}%`;
                inputRangeMargin.value = `${margin}`;
            }

            setMargin(margin);
        }
    }

    // Select Churn Rate
    let progressChurnrate: HTMLDivElement | undefined;
    let inputRangeChurnrate: HTMLInputElement | undefined;
    let changeChurnrate = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressChurnrate) {
            progressChurnrate.style.width = `${value}%`;
        }
        
        setChurnrate(value);
        
    }
    let inputChurnrate = (event: InputEvent) => {
        let churnRate = Number(event.currentTarget.value);

        if(churnRate < 0 || churnRate > 100) {

        }
        else {
            if (progressChurnrate && inputRangeChurnrate) {
                progressChurnrate.style.width = `${churnRate}%`;
                inputRangeChurnrate.value = `${churnRate}`;
            }

            setChurnrate(churnRate);
        }
    }

    let createNewModel = () => {
        let create_data: CreateModelData =  {
            name: name(),
            model_type: type(),
            cac: cac(),
            arpu: arpu(),
            margin: margin(),
            churn_rate: churnrate()
        }

        
    }

    return (
        <div class="text-body flex flex-col min-w-screen min-h-screen h-screen bg-primary text-txtprimary p-10 ">
            <A href="/" class="group self-start flex items-center gap-1.5">
                <div class="w-6 h-6 relative">   
                    <img src="./src/assets/left_arrow.png" 
                        class="absolute w-6 h-6 duration-250 opacity-100 group-hover:opacity-0"/>
                    <img src="./src/assets/accent_left_arrow.png"
                        class="absolute w-6 h-6 duration-250 opacity-0 group-hover:opacity-100"/>
                </div>
                <p class="duration-250 group-hover:text-accent">Назад</p>
            </A>
            <main class="flex flex-col max-w-300 h-full mx-auto justify-around">
                <h1>✨ Новая модель</h1>
 
                <form class="flex flex-col" > 
                    <div class="flex justify-between flex-wrap">
                        <div class="flex flex-col w-[48%]">
                            <label class="text-h3">Название модели</label>
                            <input class="min-w-100 w-full border px-3 py-2 rounded-[10px] placeholder:text-h2 placeholder:text-white placeholder:font-semibold placeholder:text-center text-center font-semibold focus:border-accent ring-0" value={name()} onInput={(e) => setName(e.target.value)}/>
                        </div>
                        <div class="flex flex-col w-[48%] align-top">
                            <label class="text-h3">Тип модели</label>
                            <div class="flex justify-between min-w-100 gap-4">
                                <button onClick={(e) => selectType(e, 0)}
                                 class={"duration-250 bg-secondary p-2 min-w-22 w-full border rounded-[10px] " + (type() == 0 ? "border-accent text-accent" : "  ")}>SaaS</button>
                                <button onClick={(e) => selectType(e, 1)} class={"duration-250 bg-secondary p-2 w-full min-w-22 border rounded-[10px] " + (type() == 1 ? "border-accent text-accent" : "  ")}>E-com</button>
                                <button onClick={(e) => selectType(e, 2)} class={"duration-250 bg-secondary p-2 w-full min-w-22 border rounded-[10px] " + (type() == 2 ? "border-accent text-accent" : "  ")}>Market</button>
                                <button onClick={(e) => selectType(e, 3)} class={"duration-250 bg-secondary p-2 w-full min-w-22 border rounded-[10px] " + (type() == 3 ? "border-accent text-accent" : "  ")}>Custom</button>
                            </div>
                        </div>
                        <div class="w-full my-12 h-0.5 bg-white overflow-y-visible relative">
                            <h4 class="text-center -mt-2.5 bg-primary w-max mx-auto px-2 text-body">Базовые параметры</h4>
                        </div>
                        
                        {/*CAC */}
                        <div class="w-[48%] relative">
                            <div class="flex justify-between">
                                <label class="my-auto">CAC (Стоимость привлечения)</label>
                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                    <input 
                                        id="cac-input"
                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250" 
                                        value={cac()}
                                        onInput={(e) => inputCac(e)}
                                    />
                                    <label 
                                        for="cac-input"
                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                    >
                                        {numberWithSpaces(cac()) + " Р"}
                                    </label>
                                </div>
                            </div>
                            <div class="relative w-full h-2 mt-2 mb-1">
                                <div 
                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 " 
                                    ref={progressCac}
                                    style={`width: calc(${cacProgress()}%)`}
                                />    
                                <div class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                <input
                                    step={0.01}
                                    type="range" 
                                    ref={inputRangeCac}
                                    min="0" 
                                    max="100" 
                                    value="22.36"
                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                    onInput={changeCac}
                                />
                            </div>
                            <p class="w-full text-end">100 000</p>
                        </div>

                        {/*ARPU */}
                        <div class="w-[48%] relative">
                            <div class="flex justify-between">
                                <label class="my-auto">ARPU (Средний доход с пользователя)</label>
                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                    <input 
                                        id="arpu-input"
                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250" 
                                        value={arpu()}
                                        onInput={(e) => inputArpu(e)}
                                    />
                                    <label 
                                        for="arpu-input"
                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                    >
                                        {numberWithSpaces(arpu()) + " Р"}
                                    </label>
                                </div>
                            </div>
                            <div class="relative w-full h-2 mt-2 mb-1">
                                <div 
                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 " 
                                    ref={progressArpu}
                                    style={`width: calc(${arpuProgress()}%)`}
                                />    
                                <div class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                <input
                                    step={0.01}
                                    type="range" 
                                    ref={inputRangeArpu}
                                    min="0" 
                                    max="100" 
                                    value="1"
                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                    onInput={changeArpu}
                                />
                            </div>
                            <p class="w-full text-end">1 000 000</p>
                        </div>

                        {/*Margin */}
                        <div class="w-[48%] relative">
                            <div class="flex justify-between">
                                <label class="my-auto">Margin (Маржинальность)</label>
                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                    <input 
                                        id="margin-input"
                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250" 
                                        value={margin()}
                                        onInput={(e) => inputMargin(e)}
                                    />
                                    <label 
                                        for="arpu-input"
                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                    >
                                        {margin() + " %"}
                                    </label>
                                </div>
                            </div>
                            <div class="relative w-full h-2 mt-2 mb-1">
                                <div 
                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 " 
                                    ref={progressMargin}
                                    style={`width: ${margin()}%`}
                                />    
                                <div class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                <input
                                    type="range" 
                                    ref={inputRangeMargin}
                                    min="0" 
                                    max="100" 
                                    value="15"
                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                    onInput={changeMargin}
                                />
                            </div>
                            <p class="w-full text-end">100%</p>
                        </div>

                        {/*Churn Rate */}
                        <div class="w-[48%] relative">
                            <div class="flex justify-between">
                                <label class="my-auto">Churn Rate (Отток)</label>
                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                    <input 
                                        id="churnrate-input"
                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250" 
                                        value={churnrate()}
                                        onInput={(e) => inputChurnrate(e)}
                                    />
                                    <label 
                                        for="churnrate-input"
                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                    >
                                        {churnrate() + " %"}
                                    </label>
                                </div>
                            </div>
                            <div class="relative w-full h-2 mt-2 mb-1">
                                <div 
                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 " 
                                    ref={progressChurnrate}
                                    style={`width: ${churnrate()}%`}
                                />    
                                <div class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                <input
                                    type="range" 
                                    ref={inputRangeChurnrate}
                                    min="0" 
                                    max="100" 
                                    value="4"
                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                    onInput={changeChurnrate}
                                />
                            </div>
                            <p class="w-full text-end">100%</p>
                        </div>

                    </div>
                </form>

                <button class="mt-6 self-center bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary w-127.5" onClick={() => createNewModel()}>Создать модель</button>
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
