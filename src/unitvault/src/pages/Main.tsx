import {dflt, UnitModel} from "../lib/models/unitmodel.ts";
import {createStore} from "solid-js/store";


export default function Main() {
    const [model, setModel] = createStore<UnitModel>(dflt())

    return (
      <div>
          Всё ОК!
          {model.name}
      </div>
    );
}