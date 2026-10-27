import {A, useNavigate} from "@solidjs/router";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import {createSignal, Match, Show, Switch} from "solid-js";
import {createStore} from "solid-js/store";
import {BusinessType, dflt, UnitModel} from "../lib/models/unitmodel.ts";

function numberWithSpaces(x: number): string {
  return x.toFixed(0).replace(/\B(?=(\d{3})+(?!\d))/g, " ");
}

export default function Create() {
    const navigate = useNavigate();

    const [model, setModel] = createStore<UnitModel>(dflt());
    const [name, setName] = createSignal("");
    const [type, setType] = createSignal(0);

    const [isOptional, setIsOptional] = createSignal(false);

    let selectType = (event: Event, num: number) => {
        event.preventDefault();
        setType(num);
    }

    //Select CAC
    const [cacProgress, setCacProgress] = createSignal(Math.sqrt(model.inputs.cac / 10));
    let progressCac: HTMLDivElement | undefined;
    let inputRangeCac: HTMLInputElement | undefined;
    let changeCac = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressCac) {
            progressCac.style.width = `${value}%`;
            setCacProgress(value);
        }
        
        setModel("inputs", "cac", Math.trunc(value * 10 * value));
        
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

            setModel("inputs", "cac", cac);
        }
    }

    // Select ARPU
    const [arpuProgress, setArpuProgress] = createSignal(model.inputs.arpu ?? 0);
    let progressArpu: HTMLDivElement | undefined;
    let inputRangeArpu: HTMLInputElement | undefined;
    let changeArpu = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressArpu) {
            progressArpu.style.width = `${value}%`;
            setArpuProgress(value);
        }
        
        setModel("inputs", "arpu", Math.trunc(value * 100 * value));
        
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

            setModel("inputs", "arpu", arpu);
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
        
        setModel("inputs", "margin", value);
        
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

            setModel("inputs", "margin", margin);
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
        
        setModel("inputs", "churn_rate", value);
        
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

            setModel("inputs", "churn_rate", churnRate);
        }
    }

    // Select Time Horizons
    const [timeHorizonsProgress, setTimeHorizonsProgress] = createSignal(model.inputs.time_horizons * 1.667);
    let progressTimeHorizons: HTMLDivElement | undefined;
    let inputRangeTimeHorizons: HTMLInputElement | undefined;
    let changeTimeHorizons = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressTimeHorizons) {
            progressTimeHorizons.style.width = `${(value * 1.667)}%`;
            setTimeHorizonsProgress((value * 1.667));
        }

        setModel("inputs", "time_horizons", value);

    }
    let inputTimeHorizons = (event: InputEvent) => {
        let time_horizons = Number(event.currentTarget.value);

        if(time_horizons < 0 || time_horizons > 60) {

        }
        else {
            let value = time_horizons;

            if (progressTimeHorizons && inputRangeTimeHorizons) {
                progressTimeHorizons.style.width = `${(value * 1.667)}%`;
                setTimeHorizonsProgress((value * 1.667));
                inputRangeTimeHorizons.value = `${value}`;
            }

            setModel("inputs", "time_horizons", time_horizons);
        }
    }

    // Select Fixed Cost
    const [fixedCostProgress, setFixedCostProgress] = createSignal(Math.sqrt((model.inputs.fixed_costs ?? 0) / 100));
    let progressFixedCost: HTMLDivElement | undefined;
    let inputRangeFixedCost: HTMLInputElement | undefined;
    let changeFixedCost = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressFixedCost) {
            progressFixedCost.style.width = `${value}%`;
            setFixedCostProgress(value);
        }

        setModel("inputs", "fixed_costs", Math.trunc(value * 100 * value));

    }
    let inputFixedCost = (event: InputEvent) => {
        let fixed_cost = Number(event.currentTarget.value);

        if(fixed_cost < 0 || fixed_cost > 1000000) {

        }
        else {
            let value = Math.sqrt(fixed_cost / 100);

            if (progressFixedCost && inputRangeFixedCost) {
                progressFixedCost.style.width = `${value}%`;
                setFixedCostProgress(value);
                inputRangeFixedCost.value = `${value}`;
            }

            setModel("inputs", "fixed_costs", fixed_cost);
        }
    }

    // Select Expansion Revenue
    let progressExpRevenue: HTMLDivElement | undefined;
    let inputRangeExpRevenue: HTMLInputElement | undefined;
    let changeExpRevenue = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressExpRevenue) {
            progressExpRevenue.style.width = `${value}%`;
        }

        setModel("inputs", "expansion_revenue", value);

    }
    let inputExpRevenue = (event: InputEvent) => {
        let exp_revenue = Number(event.currentTarget.value);

        if(exp_revenue < 0 || exp_revenue > 100) {

        }
        else {
            if (progressExpRevenue && inputRangeExpRevenue) {
                progressExpRevenue.style.width = `${exp_revenue}%`;
                inputRangeExpRevenue.value = `${exp_revenue}`;
            }

            setModel("inputs", "expansion_revenue", exp_revenue);
        }
    }

    // Select Discount Rate
    let progressDiscountRate: HTMLDivElement | undefined;
    let inputRangeDiscountRate: HTMLInputElement | undefined;
    let changeDiscountRate = (event: InputEvent) => {
        let value = event.currentTarget.value;
        if (progressDiscountRate) {
            progressDiscountRate.style.width = `${value}%`;
        }

        setModel("inputs", "discount_rate", value);

    }
    let inputDiscountRate = (event: InputEvent) => {
        let discount_rate = Number(event.currentTarget.value);

        if(discount_rate < 0 || discount_rate > 100) {

        }
        else {
            if (progressDiscountRate && inputRangeDiscountRate) {
                progressDiscountRate.style.width = `${discount_rate}%`;
                inputRangeDiscountRate.value = `${discount_rate}`;
            }

            setModel("inputs", "discount_rate", discount_rate);
        }
    }


    let createNewModel = () => {
        switch (model.business_type) {
            case BusinessType.SaaS:

                break;
            case BusinessType.Ecommerce:
                break;
            case BusinessType.Marketplace:
                break;
            case BusinessType.Custom:
                break;
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
                    <div class="flex justify-between flex-wrap transition-all duration-250">
                        <div class="flex flex-col w-[48%]">
                            <label class="text-h3">Название модели</label>
                            <input class="min-w-100 w-full border px-3 py-2 rounded-[10px] placeholder:text-h2 placeholder:text-white placeholder:font-semibold placeholder:text-center text-center font-semibold focus:border-accent ring-0" value={name()} onInput={(e) => setName(e.target.value)}/>
                        </div>
                        <div class="flex flex-col w-[48%] align-top">
                            <label class="text-h3">Тип модели</label>
                            <div class="flex justify-between min-w-100 gap-4">
                                <button onClick={(e) => selectType(e, 0)}
                                 class={"duration-250 bg-secondary p-2 min-w-22 w-full border rounded-[10px] " + (type() == 0 ? "border-accent text-accent" : "  ")}>SaaS</button>
                                <button onClick={(e) => selectType(e, 1)}
                                 class={"duration-250 bg-secondary p-2 w-full min-w-22 border rounded-[10px] " + (type() == 1 ? "border-accent text-accent" : "  ")}>E-com</button>
                                <button onClick={(e) => selectType(e, 2)}
                                 class={"duration-250 bg-secondary p-2 w-full min-w-22 border rounded-[10px] " + (type() == 2 ? "border-accent text-accent" : "  ")}>Market</button>
                                <button onClick={(e) => selectType(e, 3)}
                                 class={"duration-250 bg-secondary p-2 w-full min-w-22 border rounded-[10px] " + (type() == 3 ? "border-accent text-accent" : "  ")}>Custom</button>
                            </div>
                        </div>
                        <div class="w-full my-12 h-0.5 bg-white overflow-y-visible relative flex">
                            <div class="w-1/2 flex justify-end">
                                <h4 class={"duration-250 -mt-2.5 text-center w-max bg-primary px-2 hover:text-accent-hover " + (!isOptional() ? "text-accent" : "")}
                                    onClick={() => setIsOptional(false)}>Базовые параметры</h4>
                            </div>
                            <div class="w-1/2">
                                <h4 class={" duration-250 -mt-2.5 text-center w-max bg-primary px-2 hover:text-accent-hover " + (isOptional() ? "text-accent" : "")}
                                    onClick={() => setIsOptional(true)}>Дополнительные параметры</h4>
                            </div>
                        </div>

                        <Switch>
                            <Match when={!isOptional()}>
                                <Switch>
                                    <Match when={type() == 0}>
                                        {/*CAC */}
                                        <div class="w-[48%] relative">
                                            <div class="flex justify-between">
                                                <label class="my-auto">CAC (Стоимость привлечения)</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="cac-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.cac}
                                                        onInput={(e) => inputCac(e)}
                                                    />
                                                    <label
                                                        for="cac-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {numberWithSpaces(model.inputs.cac) + " Р"}
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
                                                        value={model.inputs.arpu ?? 0}
                                                        onInput={(e) => inputArpu(e)}
                                                    />
                                                    <label
                                                        for="arpu-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {numberWithSpaces(model.inputs.arpu ?? 0) + " Р"}
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
                                        <div class="w-[48%] relative mt-5">
                                            <div class="flex justify-between">
                                                <label class="my-auto">Margin (Маржинальность)</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="margin-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.margin}
                                                        onInput={(e) => inputMargin(e)}
                                                    />
                                                    <label
                                                        for="arpu-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {model.inputs.margin + " %"}
                                                    </label>
                                                </div>
                                            </div>
                                            <div class="relative w-full h-2 mt-2 mb-1">
                                                <div
                                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 "
                                                    ref={progressMargin}
                                                    style={`width: ${model.inputs.margin}%`}
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
                                        <div class="w-[48%] relative mt-5">
                                            <div class="flex justify-between">
                                                <label class="my-auto">Churn Rate (Отток)</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="churnrate-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.churn_rate ?? 0}
                                                        onInput={(e) => inputChurnrate(e)}
                                                    />
                                                    <label
                                                        for="churnrate-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {(model.inputs.churn_rate ?? "0") + " %"}
                                                    </label>
                                                </div>
                                            </div>
                                            <div class="relative w-full h-2 mt-2 mb-1">
                                                <div
                                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 "
                                                    ref={progressChurnrate}
                                                    style={`width: ${model.inputs.churn_rate ?? 0}%`}
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
                                    </Match>
                                </Switch>
                            </Match>
                            <Match when={isOptional()}>
                                <Switch>
                                    <Match when={type() == 0}>
                                        {/*Time Horizons*/}
                                        <div class="w-[48%] relative">
                                            <div class="flex justify-between">
                                                <label class="my-auto">Горизонт планирования</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="time-horizons-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.time_horizons}
                                                        onInput={(e) => inputTimeHorizons(e)}
                                                    />
                                                    <label
                                                        for="time-horizons-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {model.inputs.time_horizons + " мес."}
                                                    </label>
                                                </div>
                                            </div>
                                            <div class="relative w-full h-2 mt-2 mb-1">
                                                <div
                                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 "
                                                    ref={progressTimeHorizons}
                                                    style={`width: calc(${timeHorizonsProgress()}%)`}
                                                />
                                                <div
                                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                                <input
                                                    step={1}
                                                    type="range"
                                                    ref={inputRangeTimeHorizons}
                                                    min="0"
                                                    max="60"
                                                    value="24"
                                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                                    onInput={changeTimeHorizons}
                                                />
                                            </div>
                                            <p class="w-full text-end">60</p>
                                        </div>

                                        {/*Fixed Costs*/}
                                        <div class="w-[48%] relative">
                                            <div class="flex justify-between">
                                                <label class="my-auto">Постоянные расходы</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="fixed-costs-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.fixed_costs ?? 0}
                                                        onInput={(e) => inputFixedCost(e)}
                                                    />
                                                    <label
                                                        for="fixed-costs-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {numberWithSpaces(model.inputs.fixed_costs ?? 0) + " Р"}
                                                    </label>
                                                </div>
                                            </div>
                                            <div class="relative w-full h-2 mt-2 mb-1">
                                                <div
                                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 "
                                                    ref={progressFixedCost}
                                                    style={`width: calc(${fixedCostProgress()}%)`}
                                                />
                                                <div
                                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                                <input
                                                    step={0.01}
                                                    type="range"
                                                    ref={inputRangeFixedCost}
                                                    min="0"
                                                    max="100"
                                                    value="1"
                                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                                    onInput={changeFixedCost}
                                                />
                                            </div>
                                            <p class="w-full text-end">1 000 000</p>
                                        </div>

                                        {/*Expansion Revenue */}
                                        <div class="w-[48%] relative mt-5">
                                            <div class="flex justify-between">
                                                <label class="my-auto">Доп. выручка от существующих клиентов</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="exp-revenue-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.expansion_revenue ?? 0}
                                                        onInput={(e) => inputExpRevenue(e)}
                                                    />
                                                    <label
                                                        for="exp-revenue-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {(model.inputs.expansion_revenue ?? "0") + " %"}
                                                    </label>
                                                </div>
                                            </div>
                                            <div class="relative w-full h-2 mt-2 mb-1">
                                                <div
                                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 "
                                                    ref={progressExpRevenue}
                                                    style={`width: ${model.inputs.expansion_revenue ?? 0}%`}
                                                />
                                                <div class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                                <input
                                                    type="range"
                                                    ref={inputRangeExpRevenue}
                                                    min="0"
                                                    max="100"
                                                    value="0"
                                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                                    onInput={changeExpRevenue}
                                                />
                                            </div>
                                            <p class="w-full text-end">100%</p>
                                        </div>

                                        {/*Discount Revenue */}
                                        <div class="w-[48%] relative mt-5">
                                            <div class="flex justify-between">
                                                <label class="my-auto">Ставка дисконтирования</label>
                                                <div class="w-1/3 h-9 bg-secondary rounded-[10px] text-center relative focus-within:[&>label]:z-0 focus-within:[&>input]:z-10">
                                                    <input
                                                        id="discount-rate-input"
                                                        class="absolute inset-0 w-full h-full text-center bg-secondary rounded-[10px] z-0 focus:outline-none focus:ring-2 focus:ring-accent duration-250"
                                                        value={model.inputs.discount_rate ?? 0}
                                                        onInput={(e) => inputDiscountRate(e)}
                                                    />
                                                    <label
                                                        for="discount-rate-input"
                                                        class="absolute inset-0 flex items-center justify-center z-10 bg-secondary rounded-[10px] pointer-events-none cursor-text duration-250"
                                                    >
                                                        {(model.inputs.discount_rate ?? "0") + " %"}
                                                    </label>
                                                </div>
                                            </div>
                                            <div class="relative w-full h-2 mt-2 mb-1">
                                                <div
                                                    class="absolute top-0 left-0 h-2 bg-accent rounded-l-lg z-10 "
                                                    ref={progressDiscountRate}
                                                    style={`width: ${model.inputs.discount_rate ?? 0}%`}
                                                />
                                                <div class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-secondary z-0"/>
                                                <input
                                                    type="range"
                                                    ref={inputRangeDiscountRate}
                                                    min="0"
                                                    max="100"
                                                    value="0"
                                                    class="absolute inset-0 w-full h-2 rounded-lg appearance-none cursor-pointer bg-transparent z-20"
                                                    onInput={changeDiscountRate}
                                                />
                                            </div>
                                            <p class="w-full text-end">100%</p>
                                        </div>
                                    </Match>
                                </Switch>
                            </Match>
                        </Switch>


                    </div>
                </form>

                <button
                    class="mt-6 self-center bg-secondary py-5 px-14 rounded-[10px] text-center text-h3 duration-250 hover:bg-tertiary w-127.5"
                    onClick={() => createNewModel()}>Создать модель
                </button>
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
